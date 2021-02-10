/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, JSRoot} from "@internal/ast";
import {CheckProvider} from "../types";
import {ModuleSignatureManager} from "../Evaluator";
import Hub from "../Hub";
import {CompilerProject} from "@internal/compiler";
import {AnyPath, UnknownPathMap} from "@internal/path";

export default async function buildGraph(
	opts: {
		ast: JSRoot;
		project?: CompilerProject;
		connected: boolean;
		provider: CheckProvider;
	},
): Promise<Hub> {
	const {ast, connected, project, provider} = opts;

	const hub = new Hub(ast, project);
	const {evaluator} = hub;
	if (provider.libs !== undefined) {
		let body: AnyJSStatement[] = [];
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
		const graphsByPath: UnknownPathMap<undefined | ModuleSignatureManager> = new UnknownPathMap();
		const graphsByKey: Map<string, undefined | ModuleSignatureManager> = new Map();

		async function getModuleSignature(
			source: string,
			origin: AnyPath,
		): Promise<undefined | ModuleSignatureManager> {
			const graphKey = `${origin.join()}:${source}`;
			if (graphsByKey.has(graphKey)) {
				// Already prepared graph
				return graphsByKey.get(graphKey);
			}

			// Query the provider for the export types
			const graph = await provider.getExportTypes(origin, source);

			// Check if the resolved graph even exists
			if (graph === undefined) {
				// TODO unknown module, create an error
				graphsByKey.set(graphKey, undefined);
				return undefined;
			}

			// Check if we've already initialised this graph before, in the case of different relative URLs
			if (graphsByPath.has(graph.path)) {
				// TODO this is pretty inefficient, we shouldn't even receive it
				const manager = graphsByPath.get(graph.path);
				graphsByKey.set(graphKey, manager);
				return manager;
			}

			// Create the graph
			const manager = evaluator.initModuleSignature(graph, getModuleSignature);
			graphsByKey.set(graphKey, manager);
			graphsByPath.set(graph.path, manager);
			await manager.init();
			return manager;
		}

		// Seed graphs
		const seedCache: Set<string> = new Set();
		await Promise.all(
			evaluator.imports.map(({source, origin: relative}) => {
				const cacheKey = `${source}:${relative}`;
				if (seedCache.has(cacheKey)) {
					return undefined;
				}

				seedCache.add(cacheKey);
				return getModuleSignature(source, relative);
			}),
		);

		// link imports
		for (const {source, importedName, origin, type} of evaluator.imports) {
			const graphKey = `${origin.join()}:${source}`;
			const graph = graphsByKey.get(graphKey);
			if (graph === undefined) {
				// unknown module, an error would have been created in the initial graph prep
				continue;
			}

			if (importedName === undefined) {
				// nothing to link here!
				continue;
			}

			type.setAbsolute(graph.path);
			graph.link(importedName, type);
		}
	}

	evaluator.intrinsics.link();
	hub.close();
	return hub;
}
