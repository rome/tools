/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory} from "./T";
import NumericLiteralT from "./NumericLiteralT";
import NumericT from "./NumericT";
import BooleanT from "./BooleanT";
import StringT from "./StringT";
import AnyT from "./AnyT";
import StringLiteralT from "./StringLiteralT";

function isNumber(t: T): boolean {
	return t instanceof NumericT || t instanceof NumericLiteralT;
}

export default class BinaryOpT extends T {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		left: T,
		operator: string,
		right: T,
	) {
		super(scope, originNode);
		this.operator = operator;
		this.left = left;
		this.right = right;
	}

	public static type = "BinaryOpT";

	private operator: string;
	private left: T;
	private right: T;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			left: addType(this.left),
			right: addType(this.right),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new BinaryOpT(
			scope,
			originNode,
			getType(data.left),
			String(data.operator),
			getType(data.right),
		);
	}

	public reduce(): T {
		const left = this.utils.reduce(this.left);
		const right = this.utils.reduce(this.right);
		const {scope, originNode, operator} = this;

		// return type
		switch (operator) {
			case // returns booleans
			"===":
			case "==":
			case "!=":
			case "!==":
			case "<":
			case "<=":
			case ">":
			case ">=":
			case "in":
			case "instanceof":
				// TODO return BooleanLiteralT in the cases whe we have all the info
				return new BooleanT(scope, originNode);

			// Returns a string or a number
			case "+":
				if (left instanceof AnyT || right instanceof AnyT) {
					return new AnyT(scope, originNode);
				} else if (
					left instanceof NumericLiteralT &&
					right instanceof NumericLiteralT
				) {
					return new NumericLiteralT(
						scope,
						originNode,
						left.value + right.value,
					);
				} else if (isNumber(left) && isNumber(right)) {
					return new NumericT(scope, originNode);
				} else if (
					left instanceof StringLiteralT &&
					right instanceof StringLiteralT
				) {
					return new StringLiteralT(scope, originNode, left.value + right.value);
				} else {
					return new StringT(scope, originNode);
				}

			// returns a number
			case "<<":
			case ">>":
			case ">>>":
			case "-":
			case "*":
			case "/":
			case "%":
			case "**":
			case "|":
			case "^":
			case "&":
				// TODO return NumericLiteralT if left/right are literals too
				return new NumericT(scope, originNode);

			default:
				throw new Error("Unknown operator");
		}
	}
}
