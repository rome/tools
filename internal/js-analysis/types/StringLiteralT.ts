/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HydrateData} from "../Evaluator";
import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import ObjT from "./ObjT";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class StringLiteralT extends ObjT {
	constructor(scope: Scope, originNode: undefined | AnyNode, value: string) {
		super(
			scope,
			originNode,
			{
				props: [],
				proto: scope.intrinsics.StringPrototype,
				calls: [],
			},
		);
		this.value = value;
	}

	public static type = "StringLiteralT";
	public value: string;

	public serialize(): HydrateData {
		return {value: this.value};
	}

	public static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return new StringLiteralT(scope, originNode, String(data.value));
	}

	public humanize(): StaticMarkup {
		let str: string = JSON.stringify(this.value);
		return markup`${str}`;
	}

	public compatibleWith(type: T): boolean {
		return type instanceof StringLiteralT && type.value === this.value;
	}
}
