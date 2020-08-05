/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HumanBuilder} from "../Utils";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory, TypeCompatibilityReturn} from "./T";
import {Scope} from "../scopes";
import {AnyNode} from "@internal/ast";

import {StaticMarkup, markup} from "@internal/markup";
export default class ObjPropT extends T {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		key: string,
		value: T,
	) {
		super(scope, originNode);
		this.key = key;
		this.value = value;
	}

	public static type = "ObjPropT";
	public key: string;
	public value: T;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			key: this.key,
			value: addType(this.value),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new ObjPropT(
			scope,
			originNode,
			String(data.key),
			getType(data.value),
		);
	}

	public compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
		if (otherType instanceof ObjPropT && otherType.key === this.key) {
			return this.utils.checkCompability(this.value, otherType.value);
		} else {
			return false;
		}
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		return markup`${this.key}: ${builder.humanize(this.value)}`;
	}
}
