/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from "@internal/diagnostics";
import {Scope} from "../../scopes";
import T from "../T";
import {orderBySimilarity} from "@internal/string-utils";
import E, {ErrorDefinition} from "./E";
import {AnyNode} from "@internal/ast";

export default class UnknownPropE extends E {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		opts: {
			object: T;
			property: T;
			key: string;
			thisKeys: Array<string>;
			protoKeys: Array<string>;
		},
	) {
		super(scope, originNode);
		this.thisKeys = opts.thisKeys;
		this.protoKeys = opts.protoKeys;
		this.allProps = [...this.thisKeys, ...this.protoKeys];
		this.key = opts.key;
		this.object = opts.object;
		this.property = opts.property;
	}

	public static type = "UnknownPropE";
	private allProps: Array<string>;
	private thisKeys: Array<string>;
	private protoKeys: Array<string>;
	private property: T;
	private object: T;
	private key: string;

	public sortProps(props: Array<string>): Array<string> {
		if (props.length === 0) {
			return props;
		}

		const ratings = orderBySimilarity(this.key, props);
		const sortedProps = ratings.map((prop) => prop.target);
		return sortedProps;
	}

	public getError(): ErrorDefinition {
		return {
			description: descriptions.TYPE_CHECK.UNKNOWN_PROP(this.key, this.allProps),
			lowerTarget: this.property,
			upperTarget: this.object,
		};
	}
}
