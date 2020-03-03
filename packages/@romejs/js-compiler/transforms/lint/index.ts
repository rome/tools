/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import defaultExportSameBasename from './defaultExportSameBasename';
import undeclaredVariables from './undeclaredVariables';
import unusedVariables from './unusedVariables';
import emptyBlocks from './emptyBlocks';
import noCompareNegZero from './noCompareNegZero';
import unsafeNegation from './unsafeNegation';
import noAsyncPromiseExecutor from './noAsyncPromiseExecutor';
import noLabelVar from './noLabelVar';
import noDuplicateKeys from './noDuplicateKeys';
import disallowVar from './disallowVar';
import disallowMultipleSpacesInRegularExpressionLiterals from './disallowMultipleSpacesInRegularExpressionLiterals';

export const lintTransforms = [
  undeclaredVariables,
  defaultExportSameBasename,
  unusedVariables,
  emptyBlocks,
  noCompareNegZero,
  unsafeNegation,
  noAsyncPromiseExecutor,
  noLabelVar,
  noDuplicateKeys,
  disallowVar,
  disallowMultipleSpacesInRegularExpressionLiterals
];
