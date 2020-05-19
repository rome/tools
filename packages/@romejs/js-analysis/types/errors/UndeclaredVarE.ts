/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";
import {Scope} from "../../scopes";
import E, {ErrorDefinition} from "./E";

export default class UndeclaredVarE extends E {
	constructor(scope: Scope, originNode: undefined | AnyNode, name: string) {
		super(scope, originNode);
		this.name = name;
	}

	static type = "UndeclaredVarE";
	name: string;

	getError(): ErrorDefinition {
		const possibleNames = this.scope.getBindingNames();
		return {
			description: descriptions.TYPE_CHECK.UNDECLARED_VARIABLE(
				this.name,
				possibleNames,
			),
			lowerTarget: this,
		};
	}
}
