/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import T from "./T";
import StringLiteralT from "./StringLiteralT";
import BooleanT from "./BooleanT";
import NumericT from "./NumericT";
import StringT from "./StringT";
import VoidT from "./VoidT";

export default class RefineTypeofT extends T {
	constructor(scope: Scope, node: AnyNode, str: T, fallback: T) {
		super(scope, node);
		this.str = str;
		this.fallback = fallback;
	}

	public static type = "RefineTypeofT";
	private str: T;
	private fallback: T;

	public reduce(): T {
		const {fallback, utils} = this;
		const str = utils.reduce(this.str);

		if (str instanceof StringLiteralT) {
			let val;

			switch (str.value) {
				case "string": {
					val = new StringT(this.scope, undefined);
					break;
				}

				case "number": {
					val = new NumericT(this.scope, undefined);
					break;
				}

				case "undefined": {
					val = new VoidT(this.scope, undefined);
					break;
				}

				case "boolean": {
					val = new BooleanT(this.scope, undefined);
					break;
				}

				case "symbol":
				case "function":
				case "object":
					// TODO
					return utils.reduce(fallback);

				default:
					// TODO complain about unknown value
					return utils.reduce(fallback);
			}

			// make sure our refinement is actually possible and matches a value in `fallback`

			// then pluck the matching type
			const types = utils.explodeUnion(fallback);
			for (const type of types) {
				if (utils.isCompatibleWith(type, val)) {
					return utils.reduce(type);
				}
			}

			// TODO complain of a missing condition
			return utils.reduce(fallback);
		}

		return utils.reduce(fallback);
	}
}
