/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HydrateData} from "../Evaluator";
import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class AnyT extends T {
	public static type = "AnyT";

	public serialize(): HydrateData {
		return {};
	}

	public static hydrate(scope: Scope, originNode: AnyNode): T {
		return new AnyT(scope, originNode);
	}

	public compatibleWith(): boolean {
		return true;
	}

	public humanize(): StaticMarkup {
		return markup`any`;
	}
}
