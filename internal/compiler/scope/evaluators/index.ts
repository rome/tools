/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import TSTypeParameterDeclaration from "./TSTypeParameterDeclaration";
import TSTypeParameter from "./TSTypeParameter";
import JSClassDeclaration from "./JSClassDeclaration";
import JSFunctionDeclaration from "./JSFunctionDeclaration";
import JSVariableDeclaration from "./JSVariableDeclaration";
import TSTypeAlias from "./TSTypeAlias";
import JSExportDefaultDeclaration from "./JSExportDefaultDeclaration";
import JSExportLocalDeclaration from "./JSExportLocalDeclaration";
import JSImportDeclaration from "./JSImportDeclaration";
import JSSwitchCase from "./JSSwitchCase";
import JSSwitchStatement from "./JSSwitchStatement";
import TSImportEqualsDeclaration from "./TSImportEqualsDeclaration";
import JSArrowFunctionExpression from "./JSArrowFunctionExpression";
import JSBlockStatement from "./JSBlockStatement";
import JSClassExpression from "./JSClassExpression";
import JSCatchClause from "./JSCatchClause";
import JSRoot from "./JSRoot";
import JSForStatement from "./JSForStatement";
import JSForOfStatement from "./JSForOfStatement";
import JSVariableDeclarationStatement from "./JSVariableDeclarationStatement";
import TSInterfaceDeclaration from "./TSInterfaceDeclaration";
import TSDeclareFunction from "./TSDeclareFunction";
import TSEnumDeclaration from "./TSEnumDeclaration";
import JSFunctionExpression from "./JSFunctionExpression";
import JSObjectMethod from "./JSObjectMethod";
import JSClassMethod from "./JSClassMethod";
import TSDeclareMethod from "./TSDeclareMethod";
import JSForInStatement from "./JSForInStatement";
import TSMappedType from "./TSMappedType";
import TSFunctionType from "./TSFunctionType";
import {AnyNode} from "@internal/ast";

export type ScopeEvaluator = {
	enter?: (node: AnyNode, parent: AnyNode, scope: Scope) => Scope;
	inject?: (node: AnyNode, parent: AnyNode, scope: Scope) => void;
};

export function createScopeEvaluator<T extends ScopeEvaluator>(obj: T): T {
	return obj;
}

const evaluators: Map<string, ScopeEvaluator> = new Map();

evaluators.set("TSFunctionType", TSFunctionType);
evaluators.set("TSMappedType", TSMappedType);
evaluators.set("TSTypeParameter", TSTypeParameter);
evaluators.set("TSTypeParameterDeclaration", TSTypeParameterDeclaration);
evaluators.set("TSDeclareMethod", TSDeclareMethod);
evaluators.set("TSDeclareFunction", TSDeclareFunction);
evaluators.set("JSClassDeclaration", JSClassDeclaration);
evaluators.set("JSFunctionDeclaration", JSFunctionDeclaration);
evaluators.set("JSVariableDeclarationStatement", JSVariableDeclarationStatement);
evaluators.set("JSVariableDeclaration", JSVariableDeclaration);
evaluators.set("JSExportDefaultDeclaration", JSExportDefaultDeclaration);
evaluators.set("JSExportLocalDeclaration", JSExportLocalDeclaration);
evaluators.set("JSFunctionExpression", JSFunctionExpression);
evaluators.set("JSImportDeclaration", JSImportDeclaration);
evaluators.set("JSSwitchCase", JSSwitchCase);
evaluators.set("JSSwitchStatement", JSSwitchStatement);
evaluators.set("TSTypeAlias", TSTypeAlias);
evaluators.set("TSImportEqualsDeclaration", TSImportEqualsDeclaration);
evaluators.set("JSArrowFunctionExpression", JSArrowFunctionExpression);
evaluators.set("JSBlockStatement", JSBlockStatement);
evaluators.set("JSClassExpression", JSClassExpression);
evaluators.set("JSCatchClause", JSCatchClause);
evaluators.set("JSRoot", JSRoot);
evaluators.set("JSForStatement", JSForStatement);
evaluators.set("JSForOfStatement", JSForOfStatement);
evaluators.set("JSForInStatement", JSForInStatement);
evaluators.set("JSClassMethod", JSClassMethod);
evaluators.set("JSObjectMethod", JSObjectMethod);
evaluators.set("TSInterfaceDeclaration", TSInterfaceDeclaration);
evaluators.set("TSEnumDeclaration", TSEnumDeclaration);

export default evaluators;
