/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import T, {SerialTypeFactory} from "./T";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import {Scope} from "../scopes";
import ClassT from "./ClassT";
import InstanceT from "./InstanceT";
import {StaticMarkup, markup} from "@internal/markup";

export default class GenericT extends T {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		name: string,
		type: T,
	) {
		super(scope, originNode);
		this.name = name;
		this.type = type;
	}

	private name: string;
	private type: T;

	public static type = "GenericT";

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			name: this.name,
			type: addType(this.type),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new GenericT(
			scope,
			originNode,
			String(data.name),
			getType(data.type),
		);
	}

	public humanize(): StaticMarkup {
		return markup`${this.name}`;
	}

	public reduce(): T {
		const type = this.utils.reduce(this.type);
		if (type instanceof ClassT) {
			return new InstanceT(this.scope, this.originNode, this.type, []);
		} else {
			return type;
		}
	}
}
