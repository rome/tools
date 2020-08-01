/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyRoot} from "@internal/ast";
import {DiagnosticSuppressions, Diagnostics} from "@internal/diagnostics";
import {AnyVisitors, TransformRequest} from "../types";
import {stageOrder, stageTransforms} from "../transforms/index";
import {Cache} from "@internal/compiler";
import CompilerContext from "../lib/CompilerContext";

type TransformResult = {
	ast: AnyRoot;
	suppressions: DiagnosticSuppressions;
	diagnostics: Diagnostics;
	cacheDependencies: Array<string>;
};

const transformCaches: Array<Cache<TransformResult>> = stageOrder.map(() =>
	new Cache()
);

export default async function transform(
	req: TransformRequest,
): Promise<TransformResult> {
	const stage = req.stage ?? "compile";

	const {options, project} = req;
	let ast: AnyRoot = req.ast;

	const cacheQuery = Cache.buildQuery(req);

	const stageNo = stageOrder.indexOf(stage);

	// Check this exact stage cache
	const stageCache = transformCaches[stageNo];
	const cached: undefined | TransformResult = stageCache.get(cacheQuery);
	if (cached !== undefined) {
		return cached;
	}

	let prevStageDiagnostics: Diagnostics = [];
	let prevStageCacheDeps: Array<string> = [];
	let suppressions: undefined | DiagnosticSuppressions;

	// Run the previous stage
	if (stageNo > 0) {
		const prevStage = await transform({...req, stage: stageOrder[stageNo - 1]});
		prevStageDiagnostics = prevStage.diagnostics;
		prevStageCacheDeps = prevStage.cacheDependencies;
		ast = prevStage.ast;
		suppressions = prevStage.suppressions;
	}

	const context = new CompilerContext({
		suppressions,
		ref: req.ref,
		ast,
		project,
		options,
		origin: {
			category: "transform",
		},
	});

	const transformFactory = stageTransforms[stage];
	const transforms = transformFactory(project.config, options);

	let visitors: AnyVisitors = await context.normalizeTransforms(transforms);

	const compiledAst = context.reduceRoot(visitors);

	const res: TransformResult = {
		suppressions: context.suppressions,
		diagnostics: [
			...prevStageDiagnostics,
			...context.diagnostics.getDiagnostics(),
		],
		cacheDependencies: [
			...prevStageCacheDeps,
			...context.getCacheDependencies(),
		],
		ast: compiledAst,
	};
	stageCache.set(cacheQuery, res);
	return res;
}
