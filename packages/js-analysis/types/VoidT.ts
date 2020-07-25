/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romefrontend/ast";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import T from "./T";
import {Markup, markup} from "@romefrontend/cli-layout";

export default class VoidT extends T {
	static type = "VoidT";

	serialize(): HydrateData {
		return {};
	}

	static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new VoidT(scope, originNode);
	}

	humanize(): Markup {
		return markup`void`;
	}
}
