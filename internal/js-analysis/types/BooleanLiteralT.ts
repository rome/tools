/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class BooleanLiteralT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, value: boolean) {
		super(scope, originNode);
		this.value = value;
	}

	public static type = "BooleanLiteralT";

	private value: boolean;

	public serialize(): HydrateData {
		return {value: this.value};
	}

	public static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return new BooleanLiteralT(scope, originNode, Boolean(data.value));
	}

	public humanize(): StaticMarkup {
		if (this.value) {
			return markup`true`;
		} else {
			return markup`false`;
		}
	}

	public compatibleWith(type: T): boolean {
		return type instanceof BooleanLiteralT && type.value === this.value;
	}
}
