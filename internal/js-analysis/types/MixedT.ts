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

export default class MixedT extends T {
	public static type = "MixedT";

	public serialize(): HydrateData {
		return {};
	}

	public static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new MixedT(scope, originNode);
	}

	public compatibleWith(): boolean {
		return false;
	}

	public humanize(): StaticMarkup {
		return markup`mixed`;
	}
}
