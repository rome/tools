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
import {Markup, markup} from "@internal/markup";

export default class OpaqueT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, name: string) {
		super(scope, originNode);
		this.name = name;
	}

	static type = "OpaqueT";
	name: string;

	serialize(): HydrateData {
		return {name: this.name};
	}

	static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return new OpaqueT(scope, originNode, String(data.name));
	}

	humanize(): Markup {
		return markup`opaque ${this.name}`;
	}

	compatibleWith(otherType: T): boolean {
		return otherType === this;
	}
}
