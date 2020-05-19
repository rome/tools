/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import T from "./T";
import NumericT from "./NumericT";
import ObjT from "./ObjT";

export default class NumericLiteralT extends ObjT {
	constructor(scope: Scope, originNode: undefined | AnyNode, value: number) {
		super(
			scope,
			originNode,
			{
				props: [],
				proto: scope.intrinsics.NumberPrototype,
				calls: [],
			},
		);
		this.value = value;
	}

	static type = "NumericLiteralT";

	value: number;

	serialize(): HydrateData {
		return {value: this.value};
	}

	static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return new NumericLiteralT(scope, originNode, Number(data.value));
	}

	humanize(): string {
		return String(this.value);
	}

	compatibleWith(type: T): boolean {
		return (
			type instanceof NumericT ||
			(type instanceof NumericLiteralT && type.value === this.value)
		);
	}
}
