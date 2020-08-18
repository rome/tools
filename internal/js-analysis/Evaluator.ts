/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSONObject} from "@internal/codec-json";
import T from "./types/T";
import {AnyNode} from "@internal/ast";
import {ModuleSignature} from "./index";
import ImportT from "./types/ImportT";
import Intrinsics from "./Intrinsics";
import Graph from "./Graph";
import Hub from "./Hub";
import {Scope} from "./scopes";
import UnknownImportE from "./types/errors/UnknownImportE";
import EmptyT from "./types/EmptyT";
import OpenT from "./types/OpenT";
import types from "./types/index";
import evaluators from "./evaluators/index";
import {ModuleSignatureType} from "./types";
import {ExtendedMap} from "@internal/collections";

export type HydrateTypeFactory = (id: unknown) => T;

export type HydrateData = JSONObject;

export type GetModuleSignature = (
	source: string,
	relative: string,
) => Promise<undefined | ModuleSignatureManager>;

export class ModuleSignatureManager {
	constructor(
		graph: ModuleSignature,
		getModuleSignature: GetModuleSignature,
		topScope: Scope,
	) {
		this.topScope = topScope;
		this.getModuleSignature = getModuleSignature;
		this.graph = graph;
		this.openTypes = new ExtendedMap("openTypes");
		this.filename = graph.filename;

		this.exportNamesToTypeId = new Map();
	}

	public filename: string;
	private getModuleSignature: GetModuleSignature;
	private topScope: Scope;
	private exportNamesToTypeId: Map<string, string>;
	private openTypes: ExtendedMap<string, OpenT>;
	private graph: ModuleSignature;

	private addAll(manager: ModuleSignatureManager) {
		for (const [name, id] of manager.exportNamesToTypeId) {
			if (name === "default") {
				// ignore `default`
				continue;
			}

			this.exportNamesToTypeId.set(name, id);

			const openType = manager.openTypes.assert(id);
			this.openTypes.set(id, openType);
		}
	}

	public async init() {
		const {graph, openTypes} = this;

		// Create initial open types for all the nodes in this graph
		for (const id in graph.types) {
			const open = new OpenT(this.topScope, undefined);
			openTypes.set(id, open);
		}

		let currGetType: undefined | ModuleSignatureType;

		// Create a factory to fetch the open ids
		const getType: HydrateTypeFactory = (id: unknown): T => {
			if (id === undefined) {
				throw new Error("expected id");
			}

			if (typeof id !== "string") {
				throw new Error("expected string id");
			}

			const type = openTypes.get(id);

			if (type === undefined) {
				throw new Error(
					`${graph.filename}: Expected type of id ${id} but it doesn't exist, serialized data: ${String(
						JSON.stringify(currGetType),
					)}`,
				);
			}

			return type;
		};

		// Fetch the graphs of `export *` dependencies, future calls to `this.getModuleSignature` will fetch from 'cache
		await Promise.all(
			graph.exports.map((def) => {
				if (def.type === "all") {
					return this.getModuleSignature(def.source, graph.filename);
				} else {
					return undefined;
				}
			}),
		);

		// Resolve all exports
		for (const def of graph.exports) {
			if (def.type === "all") {
				const manager = await this.getModuleSignature(
					def.source,
					graph.filename,
				);
				if (manager !== undefined) {
					this.addAll(manager);
				}
			} else {
				this.exportNamesToTypeId.set(def.name, def.value);
			}
		}

		// Hydrate all types in the graph and link them to their open types
		for (const id in graph.types) {
			const node = graph.types[id];
			const {origin, type, data, human} = node;
			currGetType = node;

			// Retrieve the open type
			const openT = openTypes.assert(id);

			// Get the type constructor
			const TConstructor = types.assert(type);

			// Create the type

			// @ts-ignore
			const realT = TConstructor.hydrate(
				this.topScope,
				{loc: origin},
				data,
				getType,
			);

			//
			realT.setHuman(human);

			// Link it to the open type
			openT.shouldMatch(realT);
		}
	}

	public link(importedName: string, type: ImportT): void {
		const graph = this.graph;

		// Get type id for this export
		const maybeExportId = this.exportNamesToTypeId.get(importedName);
		if (maybeExportId === undefined) {
			// Export not found in the module so let's link it to an error
			const error = new UnknownImportE(
				this.topScope,
				type.originNode,
				{
					possibleNames: Array.from(this.exportNamesToTypeId.keys()),
					importedName,
					source: graph.filename,
				},
			);
			error.shouldMatch(type);
			return;
		}

		// Retrieve the open type
		const openT = this.openTypes.assert(maybeExportId);

		// Link it to this type
		type.setResolvedType(openT);
	}
}

type Export =
	| {
			type: "local";
			name: string;
			value: T;
		}
	| {
			type: "all";
			source: string;
		};

export default class Evaluator {
	constructor(hub: Hub, filename: string) {
		this.filename = filename;
		this.nodeToType = new ExtendedMap("nodeToType");
		this.exports = [];
		this.imports = [];
		this.hub = hub;
		this.graph = hub.graph;
		// TODO we should use `ThisScope` and set it correctly to `window` or `undefined` depending on strict mode
		this.topScope = new Scope({evaluator: this});
		this.intrinsics = this.topScope.intrinsics = new Intrinsics(this.topScope);
		this.evaluatingType = undefined;
	}

	public evaluatingType: undefined | string;
	public filename: string;
	private nodeToType: ExtendedMap<AnyNode, T>;
	public hub: Hub;
	public intrinsics: Intrinsics;
	public exports: Array<Export>;
	public imports: Array<{
		relative: string;
		importedName: undefined | string;
		source: string;
		type: ImportT;
	}>;
	private topScope: Scope;
	public graph: Graph<T>;

	public initModuleSignature(
		graph: ModuleSignature,
		getModuleSignature: GetModuleSignature,
	): ModuleSignatureManager {
		return new ModuleSignatureManager(graph, getModuleSignature, this.topScope);
	}

	public seed(ast: AnyNode) {
		return this.evaluate(ast, this.topScope);
	}

	public evaluate(node: undefined | AnyNode, scope: Scope): T {
		if (node === undefined) {
			throw new Error("Expected node but received undefined");
		}

		const evaluator = evaluators.assert(node.type);
		const oldEvaluatingType = this.evaluatingType;
		this.evaluatingType = node.type;
		let type = evaluator(node, scope, this.hub);
		if (type === undefined) {
			type = new EmptyT(scope, node);
		}
		this.evaluatingType = oldEvaluatingType;
		this.nodeToType.set(node, type);
		return type;
	}

	public getTypeFromEvaluatedNode(node: AnyNode): T {
		const type = this.nodeToType.assert(node);
		return type;
	}

	public addExport(name: string, type: T) {
		this.exports.push({
			type: "local",
			name,
			value: type,
		});
	}

	public addExportAll(source: string) {
		this.exports.push({
			type: "all",
			source,
		});
	}

	public addImport(
		t: ImportT,
		opts: {
			importedName: undefined | string;
			source: string;
			relative: string;
		},
	) {
		this.imports.push({
			relative: opts.relative,
			importedName: opts.importedName,
			source: opts.source,
			type: t,
		});
	}
}
