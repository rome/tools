/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSONObject} from "@romejs/codec-json";
import T from "./types/T";
import {AnyNode} from "@romejs/ast";
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
		this.openTypes = new Map();
		this.filename = graph.filename;

		this.exportNamesToTypeId = new Map();
	}

	filename: string;
	getModuleSignature: GetModuleSignature;
	topScope: Scope;
	exportNamesToTypeId: Map<string, string>;
	openTypes: Map<string, OpenT>;
	graph: ModuleSignature;

	addAll(manager: ModuleSignatureManager) {
		for (const [name, id] of manager.exportNamesToTypeId) {
			if (name === "default") {
				// ignore `default`
				continue;
			}

			this.exportNamesToTypeId.set(name, id);

			const openType = manager.openTypes.get(id);
			if (openType === undefined) {
				throw new Error("Expected an open type");
			}
			this.openTypes.set(id, openType);
		}
	}

	async init() {
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
			const openT = openTypes.get(id);
			if (openT === undefined) {
				throw new Error("Expected an open type");
			}

			// Get the type constructor
			const TConstructor = types.get(type);
			if (TConstructor === undefined) {
				throw new Error("Expected a valid internal type constructor name");
			}

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

	link(importedName: string, type: ImportT): void {
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
		const openT = this.openTypes.get(maybeExportId);
		if (openT === undefined) {
			throw new Error("Expected an open type");
		}

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
		this.nodeToType = new Map();
		this.exports = [];
		this.imports = [];
		this.hub = hub;
		this.graph = hub.graph;
		// TODO we should use `ThisScope` and set it correctly to `window` or `undefined` depending on strict mode
		this.topScope = new Scope({evaluator: this});
		this.intrinsics = this.topScope.intrinsics = new Intrinsics(this.topScope);
		this.evaluatingType = undefined;
	}

	evaluatingType: undefined | string;
	filename: string;
	nodeToType: Map<AnyNode, T>;
	hub: Hub;
	intrinsics: Intrinsics;
	exports: Array<Export>;
	imports: Array<{
		relative: string;
		importedName: undefined | string;
		source: string;
		type: ImportT;
	}>;
	topScope: Scope;
	graph: Graph<T>;

	initModuleSignature(
		graph: ModuleSignature,
		getModuleSignature: GetModuleSignature,
	): ModuleSignatureManager {
		return new ModuleSignatureManager(graph, getModuleSignature, this.topScope);
	}

	seed(ast: AnyNode) {
		return this.evaluate(ast, this.topScope);
	}

	evaluate(node: undefined | AnyNode, scope: Scope): T {
		if (node === undefined) {
			throw new Error("Expected node but received undefined");
		}

		const evaluator = evaluators.get(node.type);
		if (evaluator === undefined) {
			throw new Error(`what is this? ${node.type}`);
		} else {
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
	}

	getTypeFromEvaluatedNode(node: AnyNode): T {
		const type = this.nodeToType.get(node);
		if (type === undefined) {
			throw new Error(
				"getTypeFromEvaluatedNode() called on a node that has not been validated yet",
			);
		} else {
			return type;
		}
	}

	addExport(name: string, type: T) {
		this.exports.push({
			type: "local",
			name,
			value: type,
		});
	}

	addExportAll(source: string) {
		this.exports.push({
			type: "all",
			source,
		});
	}

	addImport(
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
