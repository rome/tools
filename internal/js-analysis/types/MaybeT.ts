/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory, TypeCompatibilityReturn} from "./T";
import {HumanBuilder} from "../Utils";
import {Scope} from "../scopes";
import VoidT from "./VoidT";
import NullT from "./NullT";
import {StaticMarkup, markup} from "@internal/markup";

export default class MaybeT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, parent: T) {
		super(scope, originNode);
		this.parent = parent;
	}

	public static type = "MaybeT";
	private parent: T;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			parent: addType(this.parent),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new MaybeT(scope, originNode, getType(data.parent));
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		return markup`?${builder.humanize(this.parent)}`;
	}

	public explodeUnion(): Array<T> {
		return [
			new VoidT(this.scope, this.originNode),
			new NullT(this.scope, this.originNode),
			...this.utils.explodeUnion(this.parent),
		];
	}

	public compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
		if (otherType instanceof MaybeT) {
			return this.utils.checkCompability(this.parent, otherType.parent);
		} else {
			return (
				otherType instanceof VoidT ||
				otherType instanceof NullT ||
				this.utils.checkCompability(this.parent, otherType)
			);
		}
	}
}
