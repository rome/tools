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
import {AnyNode} from "@romefrontend/ast";

export default class OpenIntrinsicT extends OpenT {
	constructor(scope: Scope, originNode: undefined | AnyNode, name: string) {
		super(scope, originNode);
		this.name = name;
	}

	static type = "OpenIntrinsicT";

	name: string;

	serialize(): HydrateData {
		return {
			name: this.name,
		};
	}

	static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return scope.intrinsics.get(String(data.name));
	}

	humanize(): string {
		return "open intrinsic";
	}
}
