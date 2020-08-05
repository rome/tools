/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../../scopes";
import E, {ErrorDefinition} from "./E";
import T from "../T";
import {descriptions} from "@internal/diagnostics";

export default class NotCallableE extends E {
	constructor(scope: Scope, originNode: undefined | AnyNode, callee: T) {
		super(scope, originNode);
		this.callee = callee;
	}

	public static type = "NotCallableE";
	private callee: T;

	public getError(): ErrorDefinition {
		return {
			description: descriptions.TYPE_CHECK.NOT_CALLABLE,
			lowerTarget: this.callee,
		};
	}
}
