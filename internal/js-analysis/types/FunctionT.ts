/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory} from "./T";
import {Scope} from "../scopes";
import {HumanBuilder} from "../Utils";
import ObjT from "./ObjT";
import {StaticMarkup, markup} from "@internal/markup";

export default class FunctionT extends ObjT {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		opts: {
			params: Array<T>;
			rest: undefined | T;
			returns: T;
			props?: Array<T>;
			proto?: T;
			body?: T;
		},
	) {
		super(
			scope,
			originNode,
			{
				props: opts.props,
				proto: opts.proto,
				calls: [],
			},
		);
		this.params = opts.params;
		this.rest = opts.rest;
		this.returns = opts.returns;
		this.body = opts.body;
	}

	public static type = "FunctionT";

	private params: Array<T>;
	private rest: undefined | T;
	public returns: T;
	private body: undefined | T;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			params: this.params.map((type) => addType(type)),
			rest: this.rest ? addType(this.rest) : undefined,
			returns: addType(this.returns),
			proto: this.proto === undefined ? undefined : addType(this.proto),
			body: this.body === undefined ? undefined : addType(this.body),
			props: this.props.map((type) => addType(type)),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new FunctionT(
			scope,
			originNode,
			{
				params: Array(data.params).map((id) => getType(id)),
				rest: data.rest === undefined ? undefined : getType(data.rest),
				returns: getType(data.returns),
				props: Array(data.props).map((id) => getType(id)),
				proto: data.proto === undefined ? undefined : getType(data.proto),
				body: data.body === undefined ? undefined : getType(data.body),
			},
		);
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		return markup`(${this.params.map((param) => builder.humanize(param)).join(
			", ",
		)}) => ${builder.humanize(this.returns)}`;
	}

	public reduce(): T {
		// No body, just a type signature
		const {body} = this;
		if (body === undefined) {
			return this;
		}

		// Reduce the body and create a new function
		const reducedBody = this.utils.reduce(body);
		if (reducedBody !== body) {
			return new FunctionT(
				this.scope,
				this.originNode,
				{
					params: this.params,
					rest: this.rest,
					returns: this.returns,
					props: this.props,
					proto: this.proto,
					body: reducedBody,
				},
			);
		}

		// Already been reduced
		return this;
	}
}
