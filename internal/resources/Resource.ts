import {enhanceNodeInspectClass} from "@internal/node";
import {
	ResourceGetDetails,
	ResourceOptions,
	ResourceTree,
	ResourceTreeEntry,
	ResourcesContainer,
} from "./types";
import {extractResource} from "./index";
import {getEnvVarBoolean} from "@internal/cli-environment";
import {createResourceContainer} from "./factories";
import {AsyncVoidCallback} from "@internal/typescript-helpers";

let isDebugStack: undefined | boolean;

export default class Resource {
	constructor(opts: ResourceOptions) {
		this.options = opts;
		this.getDetails = opts.getDetails;
		this.closed = false;
		this.resources = new Set();
		this.owners = new Set();
		this[Symbol.toStringTag] = `Resource<${opts.name}>`;

		if (opts.initialResources !== undefined) {
			for (const resc of opts.initialResources) {
				this.add(resc);
			}
		}

		// If this resource isn't optional then it must be attached to something
		if (!opts.optional && this.resources.size === 0) {
			let timeoutError: undefined | Error;
			let timeoutMessage = `The resource ${opts.name} is not correctly managed as it has not been attached to a parent resource`;
			isDebugStack = isDebugStack ?? getEnvVarBoolean("ROME_RESOURCE_STACKS");if (isDebugStack) {
				timeoutError = new Error(timeoutMessage);
			}

			this.timeout = setTimeout(
				() => {
					if (timeoutError === undefined) {
						throw new Error(
							`${timeoutMessage}. Set the environment variable ROME_RESOURCE_STACKS=1 for a creation stack.`,
						);
					} else {
						throw timeoutError;
					}
				},
				0,
			);
		}
	}

	public [Symbol.toStringTag]: string;
	public getDetails: ResourceGetDetails;

	protected timeout: undefined | NodeJS.Timeout;
	private options: ResourceOptions;
	private closed: boolean;
	private _releasing: undefined | Promise<void>;
	private owners: Set<Resource>;
	private resources: Set<Resource>;

	private markAttached(): void {
		if (this.timeout !== undefined) {
			clearTimeout(this.timeout);
			this.timeout = undefined;
		}
	}

	public buildTree(): ResourceTree {
		const tree: ResourceTree = new Map();
		tree.set(this, this.buildSubTree());
		return tree;
	}

	public buildSubTree(
		seen: Map<Resource, ResourceTreeEntry> = new Map(),
	): ResourceTreeEntry {
		const children: ResourceTree = new Map();
		const entry: ResourceTreeEntry = {
			details: this.options.getDetails(),
			children,
		};

		seen.set(this, entry);

		for (const resource of this.resources) {
			const cached = seen.get(resource);
			if (cached === undefined) {
				children.set(resource, resource.buildSubTree(seen));
			} else {
				children.set(resource, cached);
			}
		}

		return entry;
	}

	public release(): Promise<void> {
		if (this.closed) {
			return Promise.resolve();
		} else {
			return this.releaseBlock();
		}
	}

	public releaseBlock(): Promise<void> {
		return this._releaseBlock(this, new Set());
	}

	private _releaseBlock(
		root: Resource = this,
		seen: Set<Resource> = new Set(),
	): Promise<void> {
		if (this._releasing) {
			return this._releasing;
		}

		if (this.closed) {
			return Promise.resolve();
		}

		const promise = this._release(root, seen);
		this._releasing = promise;
		return promise;
	}

	private async _release(root: Resource, seen: Set<Resource>): Promise<void> {
		this.closed = true;
		this.markAttached();
		seen.add(this);

		const promises = [];

		// Release all sub resources
		for (const resource of this.resources) {
			if (seen.has(resource)) {
				continue;
			}

			promises.push(resource._releaseBlock(root, seen));
		}

		// Release ourselves
		const {release} = this.options;
		if (release !== undefined) {
			promises.push(callRelease(this, release));
		}

		try {
			await Promise.all(promises);
		} finally {
			// Remove ourselves from resources we've been added as a dependency to
			for (const owner of this.owners) {
				owner.remove(this);
			}
			this.owners.clear();

			// Run finalizer
			const {finalize} = this.options;
			if (finalize !== undefined) {
				await callRelease(this, finalize);
			}

			this._releasing = undefined;
		}
	}

	public bind(rawResource: ResourcesContainer): Resource {
		const resource = extractResource(rawResource);
		this.add(resource);
		resource.add(this);
		return resource;
	}

	public createContainer(
		name: string,
		initialResources?: ResourcesContainer[],
	): Resource {
		const resource = createResourceContainer(name, {initialResources});
		this.add(resource);
		return resource;
	}

	public add(
		rawResource: ResourcesContainer,
		markAttachment: boolean = true,
	): Resource {
		const resource = extractResource(rawResource);

		if (resource.closed || this.closed || this.resources.has(resource)) {
			return resource;
		}

		if (markAttachment) {
			resource.markAttached();
		}

		this.resources.add(resource);
		resource.owners.add(this);

		return resource;
	}

	public remove(rawResource: ResourcesContainer): void {
		this.resources.delete(extractResource(rawResource));
	}
}

function callRelease(
	resource: Resource,
	callback: AsyncVoidCallback,
): Promise<void> {
	const selfPromise: Promise<void> = new Promise((resolve) => {
		resolve(callback());
	});

	const timeout = setTimeout(
		() => {
			// TODO: Add timeout informative log
		},
		3_000,
	);

	selfPromise.finally(() => {
		clearTimeout(timeout);
	});

	return selfPromise;
}

enhanceNodeInspectClass(
	Resource,
	(resc) => {
		return resc[Symbol.toStringTag];
	},
);
