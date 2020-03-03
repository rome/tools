/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import defaultExportSameBasename from './defaultExportSameBasename';
import disallowVar from './disallowVar';
import emptyBlocks from './emptyBlocks';
import sparseArray from './sparseArray';
import noAsyncPromiseExecutor from './noAsyncPromiseExecutor';
import noCompareNegZero from './noCompareNegZero';
import noCondAssign from './noCondAssign';
import noDuplicateKeys from './noDuplicateKeys';
import noLabelVar from './noLabelVar';
import undeclaredVariables from './undeclaredVariables';
import unsafeNegation from './unsafeNegation';
import unusedVariables from './unusedVariables';
import noUnsafeFinally from './noUnsafeFinally';
import noDeleteVars from './noDeleteVars';
import noTemplateCurlyInString from './noTemplateCurlyInString';

export const lintTransforms = [
  defaultExportSameBasename,
  disallowVar,
  emptyBlocks,
  sparseArray,
  noCompareNegZero,
  unsafeNegation,
  noAsyncPromiseExecutor,
  noCondAssign,
  noDuplicateKeys,
  noLabelVar,
  undeclaredVariables,
  unusedVariables,
  noUnsafeFinally,
  noDeleteVars,
  noTemplateCurlyInString,
];
