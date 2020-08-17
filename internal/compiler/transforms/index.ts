/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectConfig} from "@internal/project";
import {
	AnyVisitors,
	CompilerOptions,
	TransformStageFactories,
	TransformStageName,
	Transforms,
} from "../types";
import classProperties from "./compile/transpile/classProperties";
import paramlessCatch from "./compile/transpile/paramlessCatch";
import optionalChaining from "./compile/transpile/optionalChaining";
import nullishCoalescing from "./compile/transpile/nullishCoalescing";
import callSpread from "./compile/transpile/callSpread";
import templateLiterals from "./compile/transpile/templateLiterals";
import objectSpread from "./compile/transpile/objectSpread";
import enums from "./compile/transpile/enums";
import optimizeImports from "./compile/validation/optimizeImports";
import optimizeExports from "./compile/validation/optimizeExports";
import jsx from "./compile/jsx";
import assetTransform from "./compileForBundle/assetTransform";
import cjsRootTransform from "./compileForBundle/modern/cjsRootTransform";
import esToRefTransform from "./compileForBundle/modern/esToRefTransform";
import requireRewriteTransform from "./compileForBundle/modern/requireRewriteTransform";
//import magicCJSTransform from "./compileForBundle/legacy/magicCJSTransform";
//import inlineRequiresTransform from "./compileForBundle/legacy/inlineRequiresTransform";
//import esToCJSTransform from "./compileForBundle/legacy/esToCJSTransform";
import metaPropertyTransform from "./compileForBundle/metaPropertyTransform";
import scopedRomeTransform from "./compileForBundle/scopedRomeTransform";
import asyncImportTransform from "./compileForBundle/asyncImportTransform";
import inlineEnv from "./compileForBundle/inlineEnv";
import {commentInjectorVisitor, variableInjectorVisitor} from "./helpers";

export const stageOrder: Array<TransformStageName> = [
	"pre",
	"compile",
	"compileForBundle",
];

export const helperVisitors: AnyVisitors = [
	variableInjectorVisitor,
	commentInjectorVisitor,
];

export const stageTransforms: TransformStageFactories = {
	// These may effect dependency analysis
	pre: () => [optimizeImports, optimizeExports, jsx],
	compile: () => [
		paramlessCatch,
		optionalChaining,
		nullishCoalescing,
		objectSpread,
		classProperties,
		templateLiterals,
		callSpread,
		enums,
	],
	compileForBundle: (projectConfig: ProjectConfig, options: CompilerOptions) => {
		const opts = options.bundle;
		if (opts === undefined) {
			throw new Error("Expected bundle options for compileForBundle stage");
		}

		const transforms: Transforms = [];

		if (opts.assetPath !== undefined) {
			transforms.push(assetTransform);
		}
		transforms.push(metaPropertyTransform);
		transforms.push(asyncImportTransform);
		transforms.push(scopedRomeTransform);
		transforms.push(inlineEnv);

		//if (opts.mode === "modern") {
		transforms.push(requireRewriteTransform);
		transforms.push(
			opts.analyze.moduleType === "cjs" ? cjsRootTransform : esToRefTransform,
		);
		//} else {
		//transforms.push(inlineRequiresTransform);
		//transforms.push(esToCJSTransform);
		//transforms.push(magicCJSTransform);
		//}

		return transforms;
	},
};
