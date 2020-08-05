/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../scopes";
import {HydrateData} from "../Evaluator";
import T from "./T";
import OpenT from "./OpenT";
import {AnyNode} from "@internal/ast";
import {StaticMarkup, markup} from "@internal/markup";

export default class OpenIntrinsicT extends OpenT {
	constructor(scope: Scope, originNode: undefined | AnyNode, name: string) {
		super(scope, originNode);
		this.name = name;
	}

	public static type = "OpenIntrinsicT";

	private name: string;

	public serialize(): HydrateData {
		return {
			name: this.name,
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return scope.intrinsics.get(String(data.name));
	}

	public humanize(): StaticMarkup {
		return markup`open intrinsic`;
	}
}
