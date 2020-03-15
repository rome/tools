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
import duplicateImportSource from './duplicateImportSource';
import emptyBlocks from './emptyBlocks';
import getterReturn from './getterReturn';
import noAsyncPromiseExecutor from './noAsyncPromiseExecutor';
import noCompareNegZero from './noCompareNegZero';
import noCondAssign from './noCondAssign';
import noDeleteVars from './noDeleteVars';
import noDebugger from './noDebugger';
import noDupeArgs from './noDupeArgs';
import noDuplicateKeys from './noDuplicateKeys';
import noEmptyCharacterClass from './noEmptyCharacterClass';
import noFunctionAssign from './noFunctionAssign';
import noImportAssign from './noImportAssign';
import noLabelVar from './noLabelVar';
import noShadowRestrictedNames from './noShadowRestrictedNames';
import noTemplateCurlyInString from './noTemplateCurlyInString';
import noUnsafeFinally from './noUnsafeFinally';
import preferFunctionDeclarations from './preferFunctionDeclarations';
import sparseArray from './sparseArray';
import undeclaredVariables from './undeclaredVariables';
import unsafeNegation from './unsafeNegation';
import unusedVariables from './unusedVariables';

// Add transforms in alphabetical order.
export const lintTransforms = [
  defaultExportSameBasename,
  disallowMultipleSpacesInRegularExpressionLiterals,
  disallowVar,
  duplicateImportSource,
  emptyBlocks,
  getterReturn,
  noAsyncPromiseExecutor,
  noCompareNegZero,
  noCondAssign,
  noDebugger,
  noDeleteVars,
  noDupeArgs,
  noDuplicateKeys,
  noEmptyCharacterClass,
  noFunctionAssign,
  noImportAssign,
  noLabelVar,
  noShadowRestrictedNames,
  noTemplateCurlyInString,
  noUnsafeFinally,
  preferFunctionDeclarations,
  sparseArray,
  undeclaredVariables,
  unsafeNegation,
  unusedVariables,
];
