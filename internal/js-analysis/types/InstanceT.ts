/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import T, {SerialTypeFactory, TypeCompatibilityReturn} from "./T";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import {Scope} from "../scopes";
import {HumanBuilder} from "../Utils";
import StringLiteralT from "./StringLiteralT";
import GetPropT from "./GetPropT";
import ObjT from "./ObjT";
import {StaticMarkup, markup} from "@internal/markup";

export default class InstanceT extends ObjT {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		target: T,
		typeParameters: Array<T>,
	) {
		const prototype = new GetPropT(
			scope,
			originNode,
			target,
			new StringLiteralT(scope, originNode, "prototype"),
		);
		super(
			scope,
			originNode,
			{
				props: [],
				proto: prototype,
				calls: [],
			},
		);

		this.typeParameters = typeParameters;
		this.target = target;
	}

	private typeParameters: Array<T>;
	private target: T;

	public static type = "InstanceT";

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			target: addType(this.target),
			params: this.typeParameters.map((type) => addType(type)),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new InstanceT(
			scope,
			originNode,
			getType(data.target),
			Array(data.params).map((id) => getType(id)),
		);
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		const name = builder.humanize(this.target);
		const typeParams = this.typeParameters;
		if (typeParams.length === 0) {
			return markup`${name}`;
		} else {
			return markup`${name}<${typeParams.map((param) => builder.humanize(param)).join(
				", ",
			)}>`;
		}
	}

	public compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
		return (
			otherType instanceof InstanceT &&
			this.utils.checkCompability(this.target, otherType.target)
		);
	}
}
