/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import ClassDeclaration from "./ClassDeclaration";
import FunctionDeclaration from "./FunctionDeclaration";
import VariableDeclaration from "./VariableDeclaration";
import TypeAliasTypeAnnotation from "./TypeAliasTypeAnnotation";
import ExportDefaultDeclaration from "./ExportDefaultDeclaration";
import ExportLocalDeclaration from "./ExportLocalDeclaration";
import ImportDeclaration from "./ImportDeclaration";
import SwitchCase from "./SwitchCase";
import SwitchStatement from "./SwitchStatement";
import TSImportEqualsDeclaration from "./TSImportEqualsDeclaration";
import ArrowFunctionExpression from "./ArrowFunctionExpression";
import BlockStatement from "./BlockStatement";
import ClassExpression from "./ClassExpression";
import CatchClause from "./CatchClause";
import Program from "./Program";
import ForStatement from "./ForStatement";
import ForOfStatement from "./ForOfStatement";
import VariableDeclarationStatement from "./VariableDeclarationStatement";
import TSInterfaceDeclaration from "./TSInterfaceDeclaration";
import TSDeclareFunction from "./TSDeclareFunction";
import FunctionHead from "./FunctionHead";
import {AnyNode} from "@romejs/js-ast";

type ScopeEvaluator = {
	creator: boolean;

	// rome-ignore lint/noExplicitAny
	build: (node: any, parent: AnyNode, scope: Scope) => void | Scope;
};

const evaluators: Map<string, ScopeEvaluator> = new Map();

evaluators.set("FunctionHead", FunctionHead);
evaluators.set("TSDeclareFunction", TSDeclareFunction);
evaluators.set("ClassDeclaration", ClassDeclaration);
evaluators.set("FunctionDeclaration", FunctionDeclaration);
evaluators.set("VariableDeclarationStatement", VariableDeclarationStatement);
evaluators.set("VariableDeclaration", VariableDeclaration);
evaluators.set("TypeAliasTypeAnnotation", TypeAliasTypeAnnotation);
evaluators.set("ExportDefaultDeclaration", ExportDefaultDeclaration);
evaluators.set("ExportLocalDeclaration", ExportLocalDeclaration);
evaluators.set("ImportDeclaration", ImportDeclaration);
evaluators.set("SwitchCase", SwitchCase);
evaluators.set("SwitchStatement", SwitchStatement);
evaluators.set("TypeAliasTypeAnnotation", TypeAliasTypeAnnotation);
evaluators.set("TSImportEqualsDeclaration", TSImportEqualsDeclaration);
evaluators.set("ArrowFunctionExpression", ArrowFunctionExpression);
evaluators.set("BlockStatement", BlockStatement);
evaluators.set("ClassExpression", ClassExpression);
evaluators.set("CatchClause", CatchClause);
evaluators.set("Program", Program);
evaluators.set("ForStatement", ForStatement);
evaluators.set("ForOfStatement", ForOfStatement);
evaluators.set("ForInStatement", ForOfStatement);
evaluators.set("TSInterfaceDeclaration", TSInterfaceDeclaration);

export default evaluators;
