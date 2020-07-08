/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romefrontend/ast";
import {HydrateData} from "../Evaluator";
import BooleanLiteralT from "./BooleanLiteralT";
import {Scope} from "../scopes";
import T from "./T";

export default class BooleanT extends T {
	static type = "BooleanT";

	serialize(): HydrateData {
		return {};
	}

	static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new BooleanT(scope, originNode);
	}

	humanize(): string {
		return "boolean";
	}

	compatibleWith(type: T): boolean {
		// A boolean literal can flow into a generic boolean
		return type instanceof BooleanT || type instanceof BooleanLiteralT;
	}
}
