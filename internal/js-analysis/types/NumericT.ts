/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import NumericLiteralT from "./NumericLiteralT";
import ObjT from "./ObjT";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class NumericT extends ObjT {
	constructor(scope: Scope, originNode: undefined | AnyNode) {
		super(
			scope,
			originNode,
			{
				props: [],
				proto: scope.intrinsics.NumberPrototype,
				calls: [],
			},
		);
	}

	public static type = "NumericT";

	public serialize(): HydrateData {
		return {};
	}

	public static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new NumericT(scope, originNode);
	}

	public humanize(): StaticMarkup {
		return markup`number`;
	}

	public compatibleWith(type: T): boolean {
		// a numeric literal can flow into a generic number
		return type instanceof NumericT || type instanceof NumericLiteralT;
	}
}
