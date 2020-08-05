/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import E, {ErrorDefinition} from "./E";
import T from "../T";
import {AnyNode} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export default class MissingUnionE extends E {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		target: T,
		union: T,
		missing: Array<T>,
	) {
		super(scope, originNode);
		this.target = target;
		this.union = union;
		this.missing = missing;
	}

	public static type = "MissingUnionE";
	private target: T;
	public union: T;
	private missing: Array<T>;

	public getError(): ErrorDefinition {
		return {
			description: descriptions.TYPE_CHECK.MISSING_CONDITION(
				this.missing.map((type) => this.utils.humanize(type)),
			),
			lowerTarget: this.target,
		};
	}
}
