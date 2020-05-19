/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HydrateData} from "../Evaluator";
import {AnyNode} from "@romejs/ast";
import {Scope} from "../scopes";
import T from "./T";

export default class AnyT extends T {
	static type = "AnyT";

	serialize(): HydrateData {
		return {};
	}

	static hydrate(scope: Scope, originNode: AnyNode): T {
		return new AnyT(scope, originNode);
	}

	compatibleWith(): boolean {
		return true;
	}

	humanize(): string {
		return "any";
	}
}
