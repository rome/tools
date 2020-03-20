/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import ClassDeclaration from './ClassDeclaration';
import FunctionDeclaration from './FunctionDeclaration';
import VariableDeclaration from './VariableDeclaration';
import TypeAliasTypeAnnotation from './TypeAliasTypeAnnotation';
import ExportDefaultDeclaration from './ExportDefaultDeclaration';
import ExportLocalDeclaration from './ExportLocalDeclaration';
import ImportDeclaration from './ImportDeclaration';
import FlowTypeParameterDeclaration from './FlowTypeParameterDeclaration';
import FlowDeclareExportNamed from './FlowDeclareExportNamed';
import SwitchCase from './SwitchCase';
import SwitchStatement from './SwitchStatement';
import FlowInterfaceDeclaration from './FlowInterfaceDeclaration';
import FlowOpaqueType from './FlowOpaqueType';
import FlowDeclareOpaqueType from './FlowDeclareOpaqueType';
import FlowDeclareFunction from './FlowDeclareFunction';
import FlowDeclareClass from './FlowDeclareClass';
import TSImportEqualsDeclaration from './TSImportEqualsDeclaration';
import ArrowFunctionExpression from './ArrowFunctionExpression';
import ClassMethod from './ClassMethod';
import FunctionExpression from './FunctionExpression';
import ObjectMethod from './ObjectMethod';
import BlockStatement from './BlockStatement';
import ClassExpression from './ClassExpression';
import CatchClause from './CatchClause';
import Program from './Program';
import ForStatement from './ForStatement';
import ForOfStatement from './ForOfStatement';
import ForInStatement from './ForOfStatement';
import VariableDeclarationStatement from './VariableDeclarationStatement';
import TSInterfaceDeclaration from './TSInterfaceDeclaration';
import TSDeclareFunction from './TSDeclareFunction';
import {AnyNode} from '@romejs/js-ast';

type ScopeEvaluator = {
  creator: boolean;

  // rome-suppress lint/noExplicitAny
  build: (node: any, parent: AnyNode, scope: Scope) => void | Scope;
};

const evaluators: Map<string, ScopeEvaluator> = new Map();

evaluators.set('TSDeclareFunction', TSDeclareFunction);
evaluators.set('ClassDeclaration', ClassDeclaration);
evaluators.set('FunctionDeclaration', FunctionDeclaration);
evaluators.set('VariableDeclarationStatement', VariableDeclarationStatement);
evaluators.set('VariableDeclaration', VariableDeclaration);
evaluators.set('TypeAliasTypeAnnotation', TypeAliasTypeAnnotation);
evaluators.set('ExportDefaultDeclaration', ExportDefaultDeclaration);
evaluators.set('ExportLocalDeclaration', ExportLocalDeclaration);
evaluators.set('ImportDeclaration', ImportDeclaration);
evaluators.set('FlowTypeParameterDeclaration', FlowTypeParameterDeclaration);
evaluators.set('FlowDeclareExportNamed', FlowDeclareExportNamed);
evaluators.set('SwitchCase', SwitchCase);
evaluators.set('SwitchStatement', SwitchStatement);
evaluators.set('FlowInterfaceDeclaration', FlowInterfaceDeclaration);
evaluators.set('FlowOpaqueType', FlowOpaqueType);
evaluators.set('FlowDeclareOpaqueType', FlowDeclareOpaqueType);
evaluators.set('FlowDeclareFunction', FlowDeclareFunction);
evaluators.set('FlowDeclareClass', FlowDeclareClass);
evaluators.set('TypeAliasTypeAnnotation', TypeAliasTypeAnnotation);
evaluators.set('TSImportEqualsDeclaration', TSImportEqualsDeclaration);
evaluators.set('ArrowFunctionExpression', ArrowFunctionExpression);
evaluators.set('ClassMethod', ClassMethod);
evaluators.set('FunctionExpression', FunctionExpression);
evaluators.set('ObjectMethod', ObjectMethod);
evaluators.set('BlockStatement', BlockStatement);
evaluators.set('ClassExpression', ClassExpression);
evaluators.set('CatchClause', CatchClause);
evaluators.set('Program', Program);
evaluators.set('ForStatement', ForStatement);
evaluators.set('ForOfStatement', ForOfStatement);
evaluators.set('ForInStatement', ForInStatement);
evaluators.set('TSInterfaceDeclaration', TSInterfaceDeclaration);

export default evaluators;
