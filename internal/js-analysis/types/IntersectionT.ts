/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory, TypeCompatibilityReturn} from "./T";
import {Scope} from "../scopes";
import {HumanBuilder} from "../Utils";
import {StaticMarkup, concatMarkup, markup} from "@internal/markup";

export default class IntersectionT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, types: Array<T>) {
		super(scope, originNode);
		this.types = types;
	}

	public static type = "IntersectionT";
	private types: Array<T>;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			types: this.types.map((type) => addType(type)),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new IntersectionT(
			scope,
			originNode,
			Array(data.types).map((id) => getType(id)),
		);
	}

	public compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
		for (const type of this.types) {
			const compatibility = this.utils.checkCompability(type, otherType);
			if (compatibility.type === "incompatible") {
				return compatibility;
			}
		}
		return true;
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		return concatMarkup(
			this.types.map((type) => builder.humanize(type)),
			markup` & `,
		);
	}
}
