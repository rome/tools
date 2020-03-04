/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Add imports in alphabetical order.
import defaultExportSameBasename from './defaultExportSameBasename';
import disallowMultipleSpacesInRegularExpressionLiterals from './disallowMultipleSpacesInRegularExpressionLiterals';
import disallowVar from './disallowVar';
import emptyBlocks from './emptyBlocks';
import noAsyncPromiseExecutor from './noAsyncPromiseExecutor';
import noCompareNegZero from './noCompareNegZero';
import noCondAssign from './noCondAssign';
import noDebugger from './noDebugger';
import noDeleteVars from './noDeleteVars';
import noDupeArgs from './noDupeArgs';
import noDuplicateKeys from './noDuplicateKeys';
import noImportAssign from './noImportAssign';
import noLabelVar from './noLabelVar';
import noTemplateCurlyInString from './noTemplateCurlyInString';
import noUnsafeFinally from './noUnsafeFinally';
import sparseArray from './sparseArray';
import undeclaredVariables from './undeclaredVariables';
import unsafeNegation from './unsafeNegation';
import unusedVariables from './unusedVariables';

// Add transforms in alphabetical order.
export const lintTransforms = [
  defaultExportSameBasename,
  disallowMultipleSpacesInRegularExpressionLiterals,
  disallowVar,
  emptyBlocks,
  noAsyncPromiseExecutor,
  noCompareNegZero,
  noCondAssign,
  noDebugger,
  noDeleteVars,
  noDupeArgs,
  noDuplicateKeys,
  noImportAssign,
  noLabelVar,
  noTemplateCurlyInString,
  noUnsafeFinally,
  sparseArray,
  undeclaredVariables,
  unsafeNegation,
  unusedVariables,
];
