/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import StringLiteralT from "./StringLiteralT";
import NumericLiteralT from "./NumericLiteralT";
import BooleanLiteralT from "./BooleanLiteralT";
import StringT from "./StringT";
import BooleanT from "./BooleanT";
import NumericT from "./NumericT";
import NullT from "./NullT";
import ObjT from "./ObjT";
import VoidT from "./VoidT";
import T from "./T";

export default class TypeofT extends T {
	constructor(scope: Scope, node: undefined | AnyNode, obj: T) {
		super(scope, node);
		this.obj = obj;
	}

	public static type = "TypeofT";
	private obj: T;

	public reduce(): T {
		const types = this.utils.explodeUnion(this.obj);

		const possibleTypes = [];
		for (const rawType of types) {
			const type = this.utils.reduce(rawType);
			let typeStr;

			if (type instanceof StringT || type instanceof StringLiteralT) {
				typeStr = "string";
			}

			if (type instanceof NumericT || type instanceof NumericLiteralT) {
				typeStr = "number";
			}

			if (type instanceof BooleanT || type instanceof BooleanLiteralT) {
				typeStr = "boolean";
			}

			if (type instanceof VoidT) {
				typeStr = "undefined";
			}

			if (type instanceof ObjT) {
				if (type.calls.length === 0) {
					typeStr = "object";
				} else {
					typeStr = "function";
				}
			}

			if (type instanceof NullT) {
				typeStr = "object";
			}

			// TODO symbol

			// TODO bigint
			if (typeStr !== undefined) {
				possibleTypes.push(
					new StringLiteralT(this.scope, this.originNode, typeStr),
				);
			}
		}

		if (possibleTypes.length === 0) {
			return new StringT(this.scope, this.originNode);
		} else {
			return this.scope.createUnion(possibleTypes, this.originNode);
		}
	}
}
