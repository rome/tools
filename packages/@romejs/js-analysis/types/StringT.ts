/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../scopes";
import {HydrateData} from "../Evaluator";
import {AnyNode} from "@romejs/ast";
import StringLiteralT from "./StringLiteralT";
import ObjT from "./ObjT";
import T from "./T";

export default class StringT extends ObjT {
	constructor(scope: Scope, originNode: undefined | AnyNode) {
		super(
			scope,
			originNode,
			{
				props: [],
				proto: scope.intrinsics.StringPrototype,
				calls: [],
			},
		);
	}

	static type = "StringT";

	serialize(): HydrateData {
		return {};
	}

	static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new StringT(scope, originNode);
	}

	humanize(): string {
		return "string";
	}

	compatibleWith(type: T) {
		// a string literal can flow into a generic string
		return type instanceof StringT || type instanceof StringLiteralT;
	}
}
