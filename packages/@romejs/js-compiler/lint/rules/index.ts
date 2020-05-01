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
import emptyMatches from './emptyMatches';
import getterReturn from './getterReturn';
import inconsiderateLanguage from './inconsiderateLanguage';
import noArguments from './noArguments';
import noAsyncPromiseExecutor from './noAsyncPromiseExecutor';
import noCatchAssign from './noCatchAssign';
import noCompareNegZero from './noCompareNegZero';
import noCondAssign from './noCondAssign';
import noDebugger from './noDebugger';
import noDelete from './noDelete';
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
import noReferenceToNonExistingGroup from './noReferenceToNonExistingGroup';
import noSetterReturn from './noSetterReturn';
import noShadowRestrictedNames from './noShadowRestrictedNames';
import noShorthandArrayType from './noShorthandArrayType';
import noTemplateCurlyInString from './noTemplateCurlyInString';
import noUnsafeFinally from './noUnsafeFinally';
import noVar from './noVar';
import preferBlockStatements from './preferBlockStatements';
import preferFunctionDeclarations from './preferFunctionDeclarations';
import preferTemplate from './preferTemplate';
import preferWhile from './preferWhile';
import restrictedGlobals from './restrictedGlobals';
import singleVarDeclarator from './singleVarDeclarator';
import sortImportExportSpecifiers from './sortImportExportSpecifiers';
import sparseArray from './sparseArray';
import undeclaredVariables from './undeclaredVariables';
import unsafeNegation from './unsafeNegation';
import unusedVariables from './unusedVariables';
import {reactLintTransforms as reactLintRules} from './react';

// Add transforms in alphabetical order.
export const lintTransforms = [
  camelCase,
  caseSingleStatement,
  defaultExportSameBasename,
  doubleEquals,
  duplicateImportSource,
  emptyBlocks,
  emptyMatches,
  getterReturn,
  inconsiderateLanguage,
  noArguments,
  noAsyncPromiseExecutor,
  noCatchAssign,
  noCompareNegZero,
  noCondAssign,
  noDebugger,
  noDelete,
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
  noMultipleSpacesInRegularExpressionLiterals,
  noReferenceToNonExistingGroup,
  noSetterReturn,
  noShadowRestrictedNames,
  noShorthandArrayType,
  noTemplateCurlyInString,
  noUnsafeFinally,
  noVar,
  preferBlockStatements,
  preferFunctionDeclarations,
  preferTemplate,
  preferWhile,
  restrictedGlobals,
  singleVarDeclarator,
  sortImportExportSpecifiers,
  sparseArray,
  undeclaredVariables,
  unsafeNegation,
  unusedVariables,
  ...reactLintRules,
];
