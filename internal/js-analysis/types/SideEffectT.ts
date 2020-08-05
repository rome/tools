/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import T from "./T";

export default class SideEffectT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, actualType: T) {
		super(scope, originNode);
		this.actualType = actualType;
	}

	public static type = "SideEffectT";

	private actualType: T;

	public reduce(): T {
		return this.utils.reduce(this.actualType);
	}
}
