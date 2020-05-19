/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import JSClassDeclaration from "./JSClassDeclaration";
import JSFunctionDeclaration from "./JSFunctionDeclaration";
import JSVariableDeclaration from "./JSVariableDeclaration";
import TSTypeAliasTypeAnnotation from "./TSTypeAliasTypeAnnotation";
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
import JSProgram from "./JSProgram";
import JSForStatement from "./JSForStatement";
import JSForOfStatement from "./JSForOfStatement";
import JSVariableDeclarationStatement from "./JSVariableDeclarationStatement";
import TSInterfaceDeclaration from "./TSInterfaceDeclaration";
import TSDeclareFunction from "./TSDeclareFunction";
import JSFunctionHead from "./JSFunctionHead";
import {AnyNode} from "@romejs/ast";

type ScopeEvaluator = {
	creator: boolean;

	// rome-ignore lint/js/noExplicitAny
	build: (node: any, parent: AnyNode, scope: Scope) => void | Scope;
};

const evaluators: Map<string, ScopeEvaluator> = new Map();

evaluators.set("JSFunctionHead", JSFunctionHead);
evaluators.set("TSDeclareFunction", TSDeclareFunction);
evaluators.set("JSClassDeclaration", JSClassDeclaration);
evaluators.set("JSFunctionDeclaration", JSFunctionDeclaration);
evaluators.set("JSVariableDeclarationStatement", JSVariableDeclarationStatement);
evaluators.set("JSVariableDeclaration", JSVariableDeclaration);
evaluators.set("JSExportDefaultDeclaration", JSExportDefaultDeclaration);
evaluators.set("JSExportLocalDeclaration", JSExportLocalDeclaration);
evaluators.set("JSImportDeclaration", JSImportDeclaration);
evaluators.set("JSSwitchCase", JSSwitchCase);
evaluators.set("JSSwitchStatement", JSSwitchStatement);
evaluators.set("TSTypeAliasTypeAnnotation", TSTypeAliasTypeAnnotation);
evaluators.set("TSImportEqualsDeclaration", TSImportEqualsDeclaration);
evaluators.set("JSArrowFunctionExpression", JSArrowFunctionExpression);
evaluators.set("JSBlockStatement", JSBlockStatement);
evaluators.set("JSClassExpression", JSClassExpression);
evaluators.set("JSCatchClause", JSCatchClause);
evaluators.set("JSProgram", JSProgram);
evaluators.set("JSForStatement", JSForStatement);
evaluators.set("JSForOfStatement", JSForOfStatement);
evaluators.set("JSForInStatement", JSForOfStatement);
evaluators.set("TSInterfaceDeclaration", TSInterfaceDeclaration);

export default evaluators;
