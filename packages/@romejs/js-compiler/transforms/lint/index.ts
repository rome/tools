/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Add imports in alphabetical order.
import defaultExportSameBasename from './defaultExportSameBasename';
import duplicateImportSource from './duplicateImportSource';
import emptyBlocks from './emptyBlocks';
import getterReturn from './getterReturn';
import noAsyncPromiseExecutor from './noAsyncPromiseExecutor';
import noCompareNegZero from './noCompareNegZero';
import noCondAssign from './noCondAssign';
import noDebugger from './noDebugger';
import noDeleteVars from './noDeleteVars';
import noDupeArgs from './noDupeArgs';
import noDuplicateCase from './noDuplicateCase';
import noDuplicateGroupNamesInRegularExpressions from './noDuplicateGroupNamesInRegularExpressions';
import noDuplicateKeys from './noDuplicateKeys';
import noEmptyCharacterClass from './noEmptyCharacterClass';
import noExplicitAny from './noExplicitAny';
import noExtraBooleanCast from './noExtraBooleanCast';
import noFunctionAssign from './noFunctionAssign';
import noImportAssign from './noImportAssign';
import noLabelVar from './noLabelVar';
import noMultipleSpacesInRegularExpressionLiterals from './noMultipleSpacesInRegularExpressionLiterals';
import noShadowRestrictedNames from './noShadowRestrictedNames';
import noTemplateCurlyInString from './noTemplateCurlyInString';
import noUnsafeFinally from './noUnsafeFinally';
import noVar from './noVar';
import preferFunctionDeclarations from './preferFunctionDeclarations';
import preferTemplate from './preferTemplate';
import sparseArray from './sparseArray';
import undeclaredVariables from './undeclaredVariables';
import unsafeNegation from './unsafeNegation';
import unusedVariables from './unusedVariables';
import singleVarDeclarator from './singleVarDeclarator';
import noReferenceToNonExistingGroup from './noReferenceToNonExistingGroup';

// Add transforms in alphabetical order.
export const lintTransforms = [
  defaultExportSameBasename,
  duplicateImportSource,
  emptyBlocks,
  getterReturn,
  noAsyncPromiseExecutor,
  noCompareNegZero,
  noCondAssign,
  noDebugger,
  noDeleteVars,
  noDupeArgs,
  noDuplicateCase,
  noDuplicateGroupNamesInRegularExpressions,
  noDuplicateKeys,
  noEmptyCharacterClass,
  noExplicitAny,
  noExtraBooleanCast,
  noFunctionAssign,
  noImportAssign,
  noLabelVar,
  noReferenceToNonExistingGroup,
  noMultipleSpacesInRegularExpressionLiterals,
  noShadowRestrictedNames,
  noTemplateCurlyInString,
  noUnsafeFinally,
  noVar,
  preferFunctionDeclarations,
  preferTemplate,
  sparseArray,
  undeclaredVariables,
  unsafeNegation,
  unusedVariables,
  noDuplicateGroupNamesInRegularExpressions,
  singleVarDeclarator,
];
