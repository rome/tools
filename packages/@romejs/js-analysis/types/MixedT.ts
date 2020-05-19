/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import T from "./T";

export default class MixedT extends T {
	static type = "MixedT";

	serialize(): HydrateData {
		return {};
	}

	static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new MixedT(scope, originNode);
	}

	compatibleWith(): boolean {
		return false;
	}

	humanize(): string {
		return "mixed";
	}
}
