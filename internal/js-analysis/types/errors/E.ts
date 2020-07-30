/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {DiagnosticDescription} from "@internal/diagnostics";
import {AnyNode} from "@internal/ast";
import AnyT from "../AnyT";
import T from "../T";
import {Markup} from "@internal/markup";

export type ErrorDefinition = {
	description: DiagnosticDescription;
	lowerTarget: T;
	upperTarget?: T;
};

export default class E extends T {
	static type = "E";

	static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
		return new AnyT(scope, originNode);
	}

	humanize(): Markup {
		return this.getError().description.message;
	}

	getError(): ErrorDefinition {
		throw new Error("unimplemented");
	}

	compatibleWith(): boolean {
		return false;
	}
}
