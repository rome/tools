/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import DependencyGraph from "./DependencyGraph";
import {BundleCompileResolvedImports} from "@internal/compiler";
import {ConstJSImportModuleKind} from "@internal/ast";
import {SourceLocation} from "@internal/parser-core";
import {
	Diagnostic,
	DiagnosticIntegrity,
	DiagnosticLocation,
	Diagnostics,
	descriptions,
} from "@internal/diagnostics";
import {ProjectDefinition} from "@internal/project";
import DependencyOrderer, {DependencyOrder} from "./DependencyOrderer";
import {
	AnalyzeDependency,
	AnalyzeExportLocal,
	AnalyzeModuleType,
	AnyAnalyzeExport,
	Server,
	WorkerAnalyzeDependencyResult,
	getFileHandlerFromPath,
} from "@internal/core";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	UIDPath,
	UIDPathMap,
} from "@internal/path";

import {ExtensionHandler} from "../../common/file-handlers/types";

import {FileReference} from "@internal/core/common/types/files";
import {ExtendedMap} from "@internal/collections";
import {mergeAnalyzeDependency} from "@internal/compiler/api/analyzeDependencies/utils";

type ResolvedImportFound = {
	type: "FOUND";
	node: DependencyNode;
	record: AnalyzeExportLocal;
};

type ResolvedImportNotFound = {
	type: "NOT_FOUND";
	node: DependencyNode;
	loc: undefined | SourceLocation;
	name: string;
};

type ResolvedImport = ResolvedImportFound | ResolvedImportNotFound;

function equalKind(
	producer: AnyAnalyzeExport,
	consumerKind: ConstJSImportModuleKind,
): boolean {
	// Allow importing functions and classes as `type` and `typeof`
	if (
		producer.type === "local" &&
		(producer.valueType === "class" || producer.valueType === "function") &&
		(consumerKind === "type" || consumerKind === "typeof")
	) {
		return true;
	}

	// You can only import a type or a class as a type
	if (producer.kind === "type") {
		return consumerKind === "type";
	}

	// You can only import a value as a value or typeof
	if (producer.kind === "value") {
		return consumerKind === "typeof" || consumerKind === "value";
	}

	return false;
}

type ResolveImportsResult = {
	diagnostics: Diagnostics;
	resolved: BundleCompileResolvedImports;
};

export default class DependencyNode {
	constructor(
		server: Server,
		graph: DependencyGraph,
		ref: FileReference,
		res: WorkerAnalyzeDependencyResult,
	) {
		this.graph = graph;

		this.project = server.projectManager.assertProjectExisting(ref.real);
		this.uid = ref.uid;
		this.path = ref.real;
		this.ref = ref;
		this.type = res.value.moduleType;

		this.usedAsync = false;
		this.all = false;
		this.relativeToAbsolutePath = new ExtendedMap("relativeToAbsolutePath");
		this.absoluteToAnalyzeDependency = new AbsoluteFilePathMap();

		this.analyze = res;

		const {handler} = getFileHandlerFromPath(ref.real, this.project.config);
		this.handler = handler;

		this.shallow = false;
	}

	public uid: UIDPath;
	public type: AnalyzeModuleType;
	public all: boolean;
	public path: AbsoluteFilePath;
	public ref: FileReference;
	public analyze: WorkerAnalyzeDependencyResult;
	public handler: undefined | ExtensionHandler;
	public usedAsync: boolean;
	public relativeToAbsolutePath: ExtendedMap<string, AbsoluteFilePath>;
	public shallow: boolean;

	private graph: DependencyGraph;
	private absoluteToAnalyzeDependency: AbsoluteFilePathMap<AnalyzeDependency>;
	private project: ProjectDefinition;
	private resolveImportsCache: undefined | ResolveImportsResult;

	public getIntegrity(): undefined | DiagnosticIntegrity {
		return this.analyze.integrity;
	}

	public setUsedAsync(usedAsync: boolean) {
		this.usedAsync = usedAsync;
		if (usedAsync) {
			this.usedAsync = true;
		}
	}

	public setAll(all: boolean) {
		if (all) {
			this.all = true;
		}
	}

	public setShallow(shallow: boolean) {
		this.shallow = shallow;
	}

	public getDependents(): DependencyNode[] {
		const dependents: DependencyNode[] = [];
		for (const node of this.graph.getNodes()) {
			if (node.absoluteToAnalyzeDependency.has(this.path)) {
				dependents.push(node);
			}
		}
		return dependents;
	}

	public addDependency(
		relative: string,
		absolute: AbsoluteFilePath,
		dep: AnalyzeDependency,
	) {
		this.relativeToAbsolutePath.set(relative, absolute);

		const existing = this.absoluteToAnalyzeDependency.get(absolute);
		if (existing !== undefined) {
			dep = mergeAnalyzeDependency(dep, existing);
		}

		this.absoluteToAnalyzeDependency.set(absolute, dep);
	}

	public getDependencyInfoFromAbsolute(
		path: AbsoluteFilePath,
	): AnalyzeDependency {
		return this.absoluteToAnalyzeDependency.assert(path);
	}

	public getNodeFromRelativeDependency(
		relative: string,
	): undefined | DependencyNode {
		const absolute = this.relativeToAbsolutePath.get(relative);
		if (absolute === undefined) {
			return undefined;
		} else {
			return this.graph.maybeGetNode(absolute);
		}
	}

	public getAbsoluteDependencies(): AbsoluteFilePath[] {
		return Array.from(this.relativeToAbsolutePath.values());
	}

	public getDependencyOrder(): DependencyOrder {
		const orderer = new DependencyOrderer(this.graph);
		return orderer.order(this.path);
	}

	// Get a list of all DependencyNodes where exports could be resolved. eg. `export *`
	private getExportedModules(
		chain: Set<DependencyNode> = new Set(),
	): Set<DependencyNode> {
		if (chain.has(this)) {
			return new Set();
		} else {
			chain.add(this);
		}

		for (const exp of this.analyze.value.exports) {
			if (
				exp.type === "externalAll" &&
				this.relativeToAbsolutePath.has(exp.source)
			) {
				this.getNodeFromRelativeDependency(exp.source)?.getExportedModules(
					chain,
				);
			}
		}

		return chain;
	}

	private getExportedNames(
		kind: ConstJSImportModuleKind,
		seen: Set<DependencyNode> = new Set(),
	): Set<string> {
		if (seen.has(this)) {
			return new Set();
		} else {
			seen.add(this);
		}

		let names: Set<string> = new Set();

		for (const exp of this.analyze.value.exports) {
			if (!equalKind(exp, kind)) {
				continue;
			}

			switch (exp.type) {
				case "local": {
					names.add(exp.name);
					break;
				}

				case "external": {
					const resolved = this.getNodeFromRelativeDependency(exp.source)?.resolveImport(
						exp.imported,
						exp.loc,
					);
					if (resolved?.type === "FOUND" && equalKind(resolved.record, kind)) {
						names.add(exp.exported);
					}
					break;
				}

				case "externalNamespace": {
					names.add(exp.exported);
					break;
				}

				case "externalAll": {
					const node = this.getNodeFromRelativeDependency(exp.source);
					if (node !== undefined) {
						names = new Set([...names, ...node.getExportedNames(kind, seen)]);
					}
					break;
				}
			}
		}

		return names;
	}

	private buildDiagnosticForUnknownExport(
		kind: ConstJSImportModuleKind,
		resolved: ResolvedImportNotFound,
	): Diagnostic {
		const location: DiagnosticLocation = {
			...resolved.loc,
			path: this.path,
			integrity: this.getIntegrity(),
		};

		const expectedName = resolved.name;
		const fromSource = resolved.node.uid;

		// Check if there was a matching local in any of the exported modules
		for (const mod of resolved.node.getExportedModules()) {
			// We use an object as a hash map so need to check for pollution
			if (
				Object.prototype.hasOwnProperty.call(
					mod.analyze.value.topLevelLocalBindings,
					expectedName,
				)
			) {
				const localLoc = mod.analyze.value.topLevelLocalBindings[expectedName];
				if (localLoc !== undefined) {
					return {
						dependencies: [{path: fromSource}],
						description: descriptions.RESOLVER.UNKNOWN_EXPORT_POSSIBLE_UNEXPORTED_LOCAL(
							expectedName,
							fromSource,
							localLoc,
						),
						location,
					};
				}
			}
		}

		return {
			dependencies: [{path: fromSource}],
			description: descriptions.RESOLVER.UNKNOWN_EXPORT(
				expectedName,
				fromSource,
				Array.from(resolved.node.getExportedNames(kind)),
				(name: string) => {
					const exportInfo = resolved.node.resolveImport(name, undefined);

					if (exportInfo.type === "NOT_FOUND") {
						throw new Error(
							`mod.resolveImport returned NOT_FOUND for an export ${name} in ${exportInfo.node.path} despite being returned by getExportedNames`,
						);
					}

					return {
						location: exportInfo.record.loc,
						source: exportInfo.node === resolved.node
							? undefined
							: exportInfo.node.path.join(),
					};
				},
			),
			location,
		};
	}

	public resolveImports(): ResolveImportsResult {
		const cached = this.resolveImportsCache;
		if (cached !== undefined) {
			return cached;
		}

		const {graph} = this;

		// Build up a map of any forwarded imports
		const resolvedImports: BundleCompileResolvedImports = new UIDPathMap();

		// Diagnostics for unknown imports
		const diagnostics: Diagnostics = [];

		// Go through all of our dependencies and check if they have any external exports to forward
		for (const absolute of this.relativeToAbsolutePath.values()) {
			const mod = graph.getNode(absolute);

			// We can't follow CJS names
			if (mod.type === "cjs") {
				continue;
			}

			const usedNames = this.getDependencyInfoFromAbsolute(absolute).names;

			// Try to resolve these exports
			for (const nameInfo of usedNames) {
				const {name, kind, loc} = nameInfo;
				if (kind === "type" || kind === "typeof") {
					// Disable resolving typed imports for now as there's ridiculous code that hides some behind $FlowFixMe
					continue;
				}

				const resolved = mod.resolveImport(name, loc);

				// Unknown import
				if (resolved.type === "NOT_FOUND") {
					diagnostics.push(this.buildDiagnosticForUnknownExport(kind, resolved));
					continue;
				}

				// If the resolved target isn't the same as the file then forward it
				if (!resolved.node.uid.equal(mod.uid)) {
					let forId = resolvedImports.get(mod.uid);
					if (forId === undefined) {
						forId = new Map();
						resolvedImports.set(mod.uid, forId);
					}
					forId.set(
						name,
						{
							id: resolved.node.uid,
							name: resolved.record.name,
						},
					);
				}
			}
		}

		const result: ResolveImportsResult = {
			resolved: resolvedImports,
			diagnostics,
		};
		this.resolveImportsCache = result;
		return result;
	}

	public resolveImport(
		name: string,
		loc: undefined | SourceLocation,
		ignoreDefault: boolean = false,
		ancestry: DependencyNode[] = [],
	): ResolvedImport {
		if (ancestry.includes(this)) {
			return {
				type: "NOT_FOUND",
				loc,
				node: this,
				name,
			};
		}

		const subAncestry: DependencyNode[] = [...ancestry, this];

		// We always want to resolve exports from the bottom up
		const exports = this.analyze.value.exports.reverse();

		for (const record of exports) {
			// When resolving exportAll we never want to include the default export of those modules
			if (record.type === "local" && record.name === "default" && ignoreDefault) {
				continue;
			}

			if (
				record.type === "local" &&
				(record.name === name || record.name === "*")
			) {
				return {
					type: "FOUND",
					node: this,
					record,
				};
			}

			if (record.type === "external" && record.exported === name) {
				const node = this.getNodeFromRelativeDependency(record.source);
				if (node !== undefined) {
					return node.resolveImport(
						record.imported,
						record.loc,
						false,
						subAncestry,
					);
				}
			}

			if (record.type === "externalAll") {
				const node = this.getNodeFromRelativeDependency(record.source);

				if (node !== undefined) {
					const resolved = node.resolveImport(
						name,
						record.loc,
						true,
						subAncestry,
					);

					if (resolved.type === "FOUND") {
						return resolved;
					}
				}
			}
		}

		return {
			type: "NOT_FOUND",
			loc,
			node: this,
			name,
		};
	}
}
