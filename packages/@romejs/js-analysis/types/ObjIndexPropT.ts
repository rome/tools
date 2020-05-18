/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import T, {SerialTypeFactory} from "./T";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import {HumanBuilder} from "../Utils";
import {Scope} from "../scopes";

export default class ObjIndexPropT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, key: T, value: T) {
		super(scope, originNode);
		this.key = key;
		this.value = value;
	}

	static type = "ObjIndexPropT";

	key: T;
	value: T;

	serialize(addType: SerialTypeFactory): HydrateData {
		return {
			key: addType(this.key),
			value: addType(this.value),
		};
	}

	static hydrate(
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

	humanize(builder: HumanBuilder): string {
		return `[${builder.humanize(this.key)}]: ${builder.humanize(this.value)}`;
	}
}
