/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// lib
export {default as CompilerContext} from './lib/CompilerContext';

export {PathOptions} from './lib/Path';
export {default as Path} from './lib/Path';

export {default as Record} from './lib/Record';
export {default as Cache} from './lib/Cache';

// methods
export * from './lint/decisions';
export {default as lint} from './lint/index';
export {default as compile} from './api/compile';
export {
	default as analyzeDependencies,
	mergeAnalyzeDependencies,
} from './api/analyzeDependencies/index';

// scope
export {default as Scope} from './scope/Scope';
export * from './scope/bindings';

// utils
export * from './constants';

export {areAnalyzeDependencyResultsEqual} from './api/analyzeDependencies/utils';
export {getPrefixedNamespace as getPrefixedBundleNamespace} from './transforms/compileForBundle/_utils';
export {default as createHook} from './api/createHook';
export {
	extractSuppressionsFromProgram,
	matchesSuppression,
} from './suppressions';

// types
export {LintResult} from './lint/index';
export {CompileResult} from './api/compile';
export {HookDescriptor} from './api/createHook';
export * from './types';
