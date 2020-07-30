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
import {Markup, markup} from "@internal/markup";

export default class BooleanLiteralT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, value: boolean) {
		super(scope, originNode);
		this.value = value;
	}

	static type = "BooleanLiteralT";

	value: boolean;

	serialize(): HydrateData {
		return {value: this.value};
	}

	static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return new BooleanLiteralT(scope, originNode, Boolean(data.value));
	}

	humanize(): Markup {
		if (this.value === true) {
			return markup`true`;
		} else {
			return markup`false`;
		}
	}

	compatibleWith(type: T): boolean {
		return type instanceof BooleanLiteralT && type.value === this.value;
	}
}
