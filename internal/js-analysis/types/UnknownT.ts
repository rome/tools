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

export default class UnknownT extends T {
	static type = "UnknownT";

	serialize(): HydrateData {
		return {};
	}

	static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new UnknownT(scope, originNode);
	}

	humanize(): Markup {
		return markup`unknown`;
	}

	compatibleWith(): boolean {
		return false;
	}
}
