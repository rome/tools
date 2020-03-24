/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstImportModuleKind, AnyNode} from '@romejs/js-ast';
import {SourceLocation} from '@romejs/parser-core';
import {TransformRequest} from '../../types';
import {
  ImportRecord,
  ExportRecord,
  EscapedCJSRefRecord,
  CJSExportRecord,
  CJSVarRefRecord,
  ESExportRecord,
  TopLevelAwaitRecord,
  ImportUsageRecord,
} from './records';
import {Context, Cache} from '@romejs/js-compiler';
import transform from '../../methods/transform';
import visitors from './visitors/index';
import {
  AnalyzeDependencyResult,
  AnalyzeDependency,
  AnyAnalyzeExport,
  AnalyzeDependencyName,
  AnalyzeDependencyImportFirstUsage,
  AnalyzeModuleType,
  AnalyzeDependencyTopLevelLocalBindings,
} from '@romejs/core';
import {descriptions} from '@romejs/diagnostics';

const analyzeCache: Cache<AnalyzeDependencyResult> = new Cache();

export default async function analyzeDependencies(
  req: TransformRequest,
): Promise<AnalyzeDependencyResult> {
  const {ast, project} = req;

  const query = Cache.buildQuery(req);
  const cached: undefined | AnalyzeDependencyResult = analyzeCache.get(query);
  if (cached) {
    return cached;
  }

  const context = new Context({
    ast,
    project,
    origin: {
      category: 'analyzeDependencies',
    },
  });
  const {ast: transformedAst} = await transform({...req, stage: 'pre'});
  context.reduce(transformedAst, visitors);

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
  let hasCJSRef = false;

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
    if (record instanceof ExportRecord && record.data.type !== 'local') {
      data = record.data;
    }

    if (data !== undefined) {
      const {kind, source} = data;
      if (kind === 'type') {
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
        type: 'local',
        loc: record.node.loc,
        kind: 'value',
        valueType: 'other',
        name: '*',
      });
    }

    if (record instanceof ImportRecord) {
      let {data} = record;

      // If this source was only ever used as a type then convert us to a value
      if (data.type === 'es' && data.kind === 'value' && sourcesUsedAsType.has(
        data.source,
      )) {
        const names: Array<AnalyzeDependencyName> = [];

        for (const name of data.names) {
          names.push({
            ...name,
            kind: 'type',
          });
        }

        data = {...data, kind: 'type', names};
      }

      // If we have multiple import records for this file, then merge them together
      const existing = dependenciesBySource.get(data.source);
      if (existing === undefined) {
        dependenciesBySource.set(data.source, data);
      } else {
        let kind: ConstImportModuleKind;
        if (data.kind === existing.kind) {
          kind = data.kind;
        } else {
          kind = 'value';
        }

        const combinedRecord: AnalyzeDependency = {
          type: data.type === 'es' && existing.type === 'es' ? 'es' : 'cjs',
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
        if (existing.kind === 'type' && data.kind === 'value') {
          dependenciesBySource.delete(data.source);
        }

        dependenciesBySource.set(data.source, combinedRecord);
      }
    } else if (record instanceof ExportRecord) {
      exports.push(record.data);
    } else if (record instanceof CJSVarRefRecord) {
      hasCJSRef = true;
    } else if (record instanceof CJSExportRecord) {
      cjsExports.push(record.node);
    } else if (record instanceof ESExportRecord) {
      // No point checking for ES imported in CJS because it would have been a syntax error
      if (record.kind === 'value') {
        esValueExports.push(record.node);
      }
    } else if (record instanceof TopLevelAwaitRecord) {
      if (firstTopAwaitLocation === undefined) {
        firstTopAwaitLocation = record.loc;
      }
    } else if (record instanceof ImportUsageRecord && record.isTop &&
      record.data.kind === 'value') {
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
  let moduleType: AnalyzeModuleType = ast.sourceType === 'script' ? 'cjs' : 'es';

  // Infer module type in legacy mode
  if (project.config.bundler.mode === 'legacy') {
    if (cjsExports.length > 0) {
      moduleType = 'cjs';
    } else if (esValueExports.length > 0) {
      moduleType = 'es';
    } else if (hasCJSRef) {
      moduleType = 'cjs';
    } else {
      moduleType = 'unknown';
    }
  }

  //
  for (const record of context.records) {
    if (record instanceof CJSVarRefRecord) {
      if (project.config.bundler.mode === 'modern' && moduleType === 'es') {
        /*context.addNodeDiagnostic(record.node, {
          category: 'analyzeDependencies',
          message: `CommonJS variable <emphasis>${
            record.node.name
          }</emphasis> is not available in an ES module`,
        });*/}
    } else if (record instanceof CJSExportRecord) {
      if (moduleType === 'es') {
        context.addNodeDiagnostic(
          record.node,
          descriptions.ANALYZE_DEPENDENCIES.CJS_EXPORT_IN_ES,
        );
      }
    }
  }

  // Add an implicit default import for CJS if there is none
  if (moduleType === 'cjs' && !hasDefaultExport) {
    exports.push({
      type: 'local',
      loc: undefined,
      kind: 'value',
      valueType: 'other',
      name: 'default',
    });
  }

  const topLevelLocalBindings: AnalyzeDependencyTopLevelLocalBindings = {};

  // Get all top level bindings
  for (const [name, binding] of context.getRootScope().evaluate(ast).getOwnBindings()) {
    topLevelLocalBindings[name] = binding.node.loc;
  }

  const res: AnalyzeDependencyResult = {
    topLevelLocalBindings,
    moduleType,
    firstTopAwaitLocation,
    exports,
    dependencies,
    importFirstUsage,
    syntax: ast.syntax,
    diagnostics: [...ast.diagnostics, ...context.diagnostics],
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
    if (exp.type === 'local' && exp.kind === 'type') {
      exports.push(exp);
    }

    // Ensure that all external exports are only reachable with `type`
    if (exp.type === 'external' || exp.type === 'externalAll') {
      exports.push({
        ...exp,
        kind: 'type',
      });
    }
  }

  return {
    ...main,
    exports,
    diagnostics: [...main.diagnostics, ...second.diagnostics],
  };
}
