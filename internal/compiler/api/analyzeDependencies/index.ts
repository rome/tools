/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, ConstJSImportModuleKind} from "@internal/ast";
import {SourceLocation} from "@internal/parser-core";
import {TransformRequest} from "../../types";
import {
	CJSExportRecord,
	CJSVarRefRecord,
	ESExportRecord,
	EscapedCJSRefRecord,
	ExportRecord,
	ImportRecord,
	ImportUsageRecord,
	TopLevelAwaitRecord,
} from "./records";
import {Cache, CompilerContext} from "@internal/compiler";
import transform from "../../methods/transform";
import visitors from "./visitors/index";
import {
	AnalyzeDependency,
	AnalyzeDependencyImportFirstUsage,
	AnalyzeDependencyName,
	AnalyzeDependencyResult,
	AnalyzeDependencyTopLevelLocalBindings,
	AnalyzeModuleType,
	AnyAnalyzeExport,
} from "@internal/core";
import {descriptions} from "@internal/diagnostics";

const analyzeCache: Cache<AnalyzeDependencyResult> = new Cache();

export default async function analyzeDependencies(
	req: TransformRequest,
): Promise<AnalyzeDependencyResult> {
	let {ast, project} = req;

	const query = Cache.buildQuery(req);
	const cached: undefined | AnalyzeDependencyResult = analyzeCache.get(query);
	if (cached) {
		return cached;
	}

	const context = new CompilerContext({
		ref: req.ref,
		ast,
		project,
		origin: {
			category: "analyzeDependencies",
		},
	});
	({ast} = await transform({...req, stage: "pre"}));
	context.reduce(ast, visitors);

	//
	const importFirstUsage: AnalyzeDependencyImportFirstUsage = [];
	const seenImportFirstUsage: Set<string> = new Set();

	// Extract records
	const exports: Array<AnyAnalyzeExport> = [];
	const dependenciesBySource: Map<string, AnalyzeDependency> = new Map();

	const esValueExports: Array<AnyNode> = [];
	const cjsExports: Array<AnyNode> = [];
	let firstTopAwaitLocation: undefined | SourceLocation;

	// TODO description
	//let hasCJSRef = false;

	// Whether we have a default export, used to automatically add one for CJS
	let hasDefaultExport = false;

	// Find the import sources that are only used as a type
	const sourcesUsedAsType: Set<string> = new Set();
	const sourcesUsedAsValue: Set<string> = new Set();
	for (const record of context.records) {
		let data;

		if (record instanceof ImportUsageRecord) {
			data = record.data;
		}

		// This has to be a separate if or else TS wont refine it...
		if (record instanceof ExportRecord && record.data.type !== "local") {
			data = record.data;
		}

		if (data !== undefined) {
			const {kind, source} = data;
			if (kind === "type") {
				sourcesUsedAsType.add(source);
			} else {
				sourcesUsedAsValue.add(source);
			}
		}
	}
	for (const source of sourcesUsedAsValue) {
		sourcesUsedAsType.delete(source);
	}

	// Process rest of the records
	for (const record of context.records) {
		if (record instanceof EscapedCJSRefRecord) {
			exports.push({
				type: "local",
				loc: record.node.loc,
				kind: "value",
				valueType: "other",
				name: "*",
			});
		}

		if (record instanceof ImportRecord) {
			let {data} = record;

			// If this source was only ever used as a type then convert us to a value
			if (
				data.type === "es" &&
				data.kind === "value" &&
				sourcesUsedAsType.has(data.source)
			) {
				const names: Array<AnalyzeDependencyName> = [];

				for (const name of data.names) {
					names.push({
						...name,
						kind: "type",
					});
				}

				data = {...data, kind: "type", names};
			}

			// If we have multiple import records for this file, then merge them together
			const existing = dependenciesBySource.get(data.source);
			if (existing === undefined) {
				dependenciesBySource.set(data.source, data);
			} else {
				let kind: ConstJSImportModuleKind;
				if (data.kind === existing.kind) {
					kind = data.kind;
				} else {
					kind = "value";
				}

				const combinedRecord: AnalyzeDependency = {
					type: data.type === "es" && existing.type === "es" ? "es" : "cjs",
					kind,
					optional: existing.optional && data.optional,
					async: existing.async || data.async,
					source: data.source,
					all: existing.all || data.all,
					names: [...existing.names, ...data.names],
					loc: existing.loc || data.loc,
				};

				// Map ordering is by insertion time, so in the case where the previous import was a type import
				// then we don't want to place our combined record in that position, it should be at the end.
				// Inserting a type import statement at the top of the file shouldn't change the execution order
				// if it was imported later
				if (existing.kind === "type" && data.kind === "value") {
					dependenciesBySource.delete(data.source);
				}

				dependenciesBySource.set(data.source, combinedRecord);
			}
		} else if (record instanceof ExportRecord) {
			exports.push(record.data);
		} else if (record instanceof CJSVarRefRecord) {
			//hasCJSRef = true;
		} else if (record instanceof CJSExportRecord) {
			cjsExports.push(record.node);
		} else if (record instanceof ESExportRecord) {
			// No point checking for ES imported in CJS because it would have been a syntax error
			if (record.kind === "value") {
				esValueExports.push(record.node);
			}
		} else if (record instanceof TopLevelAwaitRecord) {
			if (firstTopAwaitLocation === undefined) {
				firstTopAwaitLocation = record.loc;
			}
		} else if (
			record instanceof ImportUsageRecord &&
			record.isTop &&
			record.data.kind === "value"
		) {
			// Track the first reference to a value import that's not in a function
			// This is used to detect module cycles
			const {data} = record;
			const key = `${data.source}:${data.imported}`;
			if (seenImportFirstUsage.has(key)) {
				continue;
			}

			seenImportFirstUsage.add(key);
			importFirstUsage.push(data);
		}
	}

	// Build dependencies
	const dependencies: Array<AnalyzeDependency> = Array.from(
		dependenciesBySource.values(),
	);

	// Infer the module type
	let moduleType: AnalyzeModuleType = "unknown";

	if (ast.type === "JSRoot") {
		moduleType = ast.sourceType === "script" ? "cjs" : "es";
	}

	//
	for (const record of context.records) {
		if (record instanceof CJSVarRefRecord) {
			if (moduleType === "es") {
				/*context.addNodeDiagnostic(record.node, {
          category: 'analyzeDependencies',
          message: `CommonJS variable <emphasis>${
            record.node.name
          }</emphasis> is not available in an ES module`,
        });*/
			}
		} else if (record instanceof CJSExportRecord) {
			if (moduleType === "es") {
				context.addNodeDiagnostic(
					record.node,
					descriptions.ANALYZE_DEPENDENCIES.CJS_EXPORT_IN_ES,
				);
			}
		}
	}

	// Add an implicit default import for CJS if there is none
	if (moduleType === "cjs" && !hasDefaultExport) {
		exports.push({
			type: "local",
			loc: undefined,
			kind: "value",
			valueType: "other",
			name: "default",
		});
	}

	const topLevelLocalBindings: AnalyzeDependencyTopLevelLocalBindings = {};

	// Get all top level bindings
	for (const [name, binding] of context.rootScope.enterEvaluate(ast).getOwnBindings()) {
		topLevelLocalBindings[name] = binding.node.loc;
	}

	const res: AnalyzeDependencyResult = {
		topLevelLocalBindings,
		moduleType,
		firstTopAwaitLocation,
		exports,
		dependencies,
		importFirstUsage,
		diagnostics: [...ast.diagnostics, ...context.diagnostics.getDiagnostics()],
	};
	analyzeCache.set(query, res);
	return res;
}

export function mergeAnalyzeDependencies(
	main: AnalyzeDependencyResult,
	second: AnalyzeDependencyResult,
): AnalyzeDependencyResult {
	const exports: Array<AnyAnalyzeExport> = [...main.exports];

	// Take only local type exports
	for (const exp of second.exports) {
		if (exp.type === "local" && exp.kind === "type") {
			exports.push(exp);
		}

		// Ensure that all external exports are only reachable with `type`
		if (exp.type === "external" || exp.type === "externalAll") {
			exports.push({
				...exp,
				kind: "type",
			});
		}
	}

	return {
		...main,
		exports,
		diagnostics: [...main.diagnostics, ...second.diagnostics],
	};
}
