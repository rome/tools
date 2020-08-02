/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import VoidT from "./VoidT";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class EmptyT extends T {
	static type = "EmptyT";

	serialize(): HydrateData {
		return {};
	}

	static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new EmptyT(scope, originNode);
	}

	humanize(): StaticMarkup {
		return markup`empty`;
	}

	compatibleWith(otherType: T): boolean {
		return otherType instanceof EmptyT || otherType instanceof VoidT;
	}
}
