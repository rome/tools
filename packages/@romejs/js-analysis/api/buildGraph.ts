/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyStatement, Program} from '@romejs/js-ast';
import {CheckProvider} from '../types';
import Evaluator, {ModuleSignatureManager} from '../Evaluator';
import Hub from '../Hub';
import {TransformProjectDefinition} from '@romejs/js-compiler';

async function getModuleSignature(
  graphs: Map<string, undefined | ModuleSignatureManager>,
  provider: CheckProvider,
  evaluator: Evaluator,
  source: string,
  relative: string,
): Promise<undefined | ModuleSignatureManager> {
  const graphKey = `${relative}:${source}`;
  if (graphs.has(graphKey)) {
    // already prepared graph
    return graphs.get(graphKey);
  }

  // query the provider for the export types
  const graph = await provider.getExportTypes(relative, source);

  // check if the resolved graph even exists
  if (graph === undefined) {
    // TODO unknown module, create an error
    graphs.set(graphKey, undefined);
    return undefined;
  }

  // check if we've already initialised this graph before, in the case of different relative URLs
  if (graphs.has(graph.filename)) {
    // TODO this is pretty inefficient, we shouldn't even receive it
    const manager = graphs.get(graph.filename);
    graphs.set(graphKey, manager);
    return manager;
  }

  // create the graph
  const manager = evaluator.initModuleSignature(graph, getModuleSignature.bind(
    null,
    graphs,
    provider,
    evaluator,
  ));
  graphs.set(graphKey, manager);
  graphs.set(graph.filename, manager);
  await manager.init();
  return manager;
}

export default async function buildGraph(opts: {
  ast: Program;
  project: TransformProjectDefinition;
  connected: boolean;
  provider: CheckProvider;
}): Promise<Hub> {
  const {ast, connected, project, provider} = opts;

  const hub = new Hub(ast, project);
  const {evaluator} = hub;
  if (provider.libs !== undefined) {
    let body: Array<AnyStatement> = [];
    for (const ast of provider.libs) {
      body = [...body, ...ast.body];
    }
    evaluator.seed({
      ...ast,
      body,
    });
  }
  evaluator.seed(ast);

  // fetch imports
  if (connected) {
    // create graphs
    const graphs: Map<string, undefined | ModuleSignatureManager> = new Map();

    // seed graphs
    const seedCache: Set<string> = new Set();
    await Promise.all(evaluator.imports.map(({source, relative}) => {
      const cacheKey = `${source}:${relative}`;
      if (seedCache.has(cacheKey)) {
        return undefined;
      }

      seedCache.add(cacheKey);
      return getModuleSignature(graphs, provider, evaluator, source, relative);
    }));

    // link imports
    for (const {source, importedName, relative, type} of evaluator.imports) {
      const graphKey = `${relative}:${source}`;
      const graph = graphs.get(graphKey);
      if (graph === undefined) {
        // unknown module, an error would have been created in the initial graph prep
        continue;
      }

      if (importedName === undefined) {
        // nothing to link here!
        continue;
      }

      type.setAbsolute(graph.filename);
      graph.link(importedName, type);
    }
  }

  evaluator.intrinsics.link();
  hub.close();
  return hub;
}
