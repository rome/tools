/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import NotCallableE from "./errors/NotCallableE";
import FunctionT from "./FunctionT";
import ObjT from "./ObjT";
import AnyT from "./AnyT";
import E from "./errors/E";
import T from "./T";

export default class CallT extends T {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		callee: T,
		args: Array<T>,
	) {
		super(scope, originNode);
		this.callee = callee;
		this.args = args;
	}

	public static type = "CallT";

	private callee: T;
	public args: Array<T>;

	public reduce(): T {
		let callee = this.utils.reduce(this.callee);
		if (callee instanceof ObjT && callee.calls.length) {
			callee = this.utils.reduce(callee.calls[0]);
		}

		if (callee instanceof AnyT || callee instanceof E) {
			return new AnyT(this.scope, this.originNode);
		} else if (callee instanceof FunctionT) {
			return this.utils.reduce(callee.returns);
		} else {
			return new NotCallableE(this.scope, this.originNode, this.callee);
		}
	}
}
