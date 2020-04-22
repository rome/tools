/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Add imports in alphabetical order.
import camelCase from './camelCase';
import caseSingleStatement from './caseSingleStatement';
import defaultExportSameBasename from './defaultExportSameBasename';
import doubleEquals from './doubleEquals';
import duplicateImportSource from './duplicateImportSource';
import emptyBlocks from './emptyBlocks';
import getterReturn from './getterReturn';
import inconsiderateLanguage from './inconsiderateLanguage';
import noArguments from './noArguments';
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
import noInnerDeclarations from './noInnerDeclarations';
import noLabelVar from './noLabelVar';
import noMultipleSpacesInRegularExpressionLiterals from './noMultipleSpacesInRegularExpressionLiterals';
import noReferenceToNonExistingGroup from './noReferenceToNonExistingGroup';
import noSetterReturn from './noSetterReturn';
import noShadowRestrictedNames from './noShadowRestrictedNames';
import noShorthandArrayType from './noShorthandArrayType';
import noTemplateCurlyInString from './noTemplateCurlyInString';
import noUnsafeFinally from './noUnsafeFinally';
import noCatchAssign from './noCatchAssign';
import noVar from './noVar';
import preferFunctionDeclarations from './preferFunctionDeclarations';
import preferTemplate from './preferTemplate';
import singleVarDeclarator from './singleVarDeclarator';
import sparseArray from './sparseArray';
import undeclaredVariables from './undeclaredVariables';
import unsafeNegation from './unsafeNegation';
import unusedVariables from './unusedVariables';

// Add transforms in alphabetical order.
export const lintTransforms = [
  camelCase,
  defaultExportSameBasename,
  duplicateImportSource,
  caseSingleStatement,
  doubleEquals,
  emptyBlocks,
  getterReturn,
  inconsiderateLanguage,
  noArguments,
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
  noInnerDeclarations,
  noLabelVar,
  noMultipleSpacesInRegularExpressionLiterals,
  noReferenceToNonExistingGroup,
  noSetterReturn,
  noShadowRestrictedNames,
  noShorthandArrayType,
  noTemplateCurlyInString,
  noUnsafeFinally,
  noCatchAssign,
  noVar,
  preferFunctionDeclarations,
  preferTemplate,
  singleVarDeclarator,
  sparseArray,
  undeclaredVariables,
  unsafeNegation,
  unusedVariables,
];
