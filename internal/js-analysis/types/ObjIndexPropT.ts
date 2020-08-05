/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import T, {SerialTypeFactory} from "./T";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import {HumanBuilder} from "../Utils";
import {Scope} from "../scopes";
import {StaticMarkup, markup} from "@internal/markup";

export default class ObjIndexPropT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, key: T, value: T) {
		super(scope, originNode);
		this.key = key;
		this.value = value;
	}

	public static type = "ObjIndexPropT";

	public key: T;
	public value: T;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			key: addType(this.key),
			value: addType(this.value),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new ObjIndexPropT(
			scope,
			originNode,
			getType(data.key),
			getType(data.value),
		);
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		return markup`[${builder.humanize(this.key)}]: ${builder.humanize(
			this.value,
		)}`;
	}
}
