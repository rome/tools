/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import defaultExportSameBasename from './defaultExportSameBasename';
import emptyBlocks from './emptyBlocks';
import noAsyncPromiseExecutor from './noAsyncPromiseExecutor';
import noCompareNegZero from './noCompareNegZero';
import noCondAssign from './noCondAssign';
import noDuplicateKeys from './noDuplicateKeys';
import noLabelVar from './noLabelVar';
import undeclaredVariables from './undeclaredVariables';
import unsafeNegation from './unsafeNegation';
import unusedVariables from './unusedVariables';

export const lintTransforms = [
  defaultExportSameBasename,
  emptyBlocks,
  noAsyncPromiseExecutor,
  noCompareNegZero,
  noCondAssign,
  noDuplicateKeys,
  noLabelVar,
  undeclaredVariables,
  unsafeNegation,
  unusedVariables,
];
