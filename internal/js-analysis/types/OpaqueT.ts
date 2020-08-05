/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class OpaqueT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, name: string) {
		super(scope, originNode);
		this.name = name;
	}

	public static type = "OpaqueT";
	private name: string;

	public serialize(): HydrateData {
		return {name: this.name};
	}

	public static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return new OpaqueT(scope, originNode, String(data.name));
	}

	public humanize(): StaticMarkup {
		return markup`opaque ${this.name}`;
	}

	public compatibleWith(otherType: T): boolean {
		return otherType === this;
	}
}
