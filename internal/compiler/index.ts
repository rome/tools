/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// lib
export {default as CompilerContext} from "./lib/CompilerContext";

export {CompilerPathOptions} from "./lib/CompilerPath";
export {default as CompilerPath} from "./lib/CompilerPath";

export {default as Record} from "./lib/Record";
export {default as Cache} from "./lib/Cache";

export * from "./lint/decisions";
export {default as lint} from "./lint/index";
export {LintRuleName} from "./lint/rules/categories";
export {lintRuleNames} from "./lint/rules/index";
export {default as compile} from "./api/compile";

export {
	default as analyzeDependencies,
	mergeAnalyzeDependencies,
} from "./api/analyzeDependencies/index";

export {default as Scope} from "./scope/Scope";
export * from "./scope/bindings";

export * from "./utils";
export * from "./constants";

import * as signals from "./signals";
export {signals};
export {
	EnterSignal,
	ExitSignal,
	ParentSignal,
	RemoveSignal,
	ReplaceSignal,
	SkipSignal,
} from "./signals";

export {areAnalyzeDependencyResultsEqual} from "./api/analyzeDependencies/utils";
export {getPrefixedNamespace as getPrefixedBundleNamespace} from "./transforms/compileForBundle/_utils";
export {matchesSuppression} from "./suppressions";

export {LintResult} from "./lint/index";
export {CompileResult} from "./api/compile";
export * from "./types";
