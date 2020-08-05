/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import T, {SerialTypeFactory} from "./T";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import StringLiteralT from "./StringLiteralT";
import GetPropT from "./GetPropT";
import ObjPropT from "./ObjPropT";
import OpenT from "./OpenT";
import ObjT from "./ObjT";

export default class ClassT extends ObjT {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		opts: {
			_constructor: undefined | T;
			statics: Array<T>;
			instances: Array<T>;
			extends?: T;
			calls?: Array<T>;
		},
	) {
		// point `class.prototype.__proto__` to `superClass.prototype`
		let protoProp = undefined;
		if (opts.extends) {
			const originNode = opts.extends.originNode;
			protoProp = new GetPropT(
				scope,
				originNode,
				opts.extends,
				new StringLiteralT(scope, originNode, "prototype"),
			);
		}

		// create `class.prototype.constructor`
		const constructorOpen = new OpenT(scope, undefined);
		const constructorProp = new ObjPropT(
			scope,
			undefined,
			"constructor",
			constructorOpen,
		);
		const instances = [...opts.instances, constructorProp];

		// create `class.prototype`
		const protoObj = new ObjT(
			scope,
			originNode,
			{
				props: instances,
				proto: protoProp,
				calls: [],
			},
		);

		super(
			scope,
			originNode,
			{
				props: [
					...opts.statics,
					new ObjPropT(scope, originNode, "prototype", protoObj),
				],
				proto: opts.extends,
				calls: opts.calls === undefined ? [] : opts.calls,
			},
		);

		constructorOpen.shouldMatch(this);

		this._constructor = opts._constructor;
		this._statics = opts.statics;
		this._instances = opts.instances;
		this._extends = opts.extends;
	}

	public static type = "ClassT";

	private _statics: Array<T>;
	private _instances: Array<T>;
	private _extends: undefined | T;
	private _constructor: undefined | T;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			constructor: this._constructor === undefined
				? undefined
				: addType(this._constructor),
			statics: this._statics.map((type) => addType(type)),
			instances: this._instances.map((type) => addType(type)),
			extends: this._extends === undefined ? undefined : addType(this._extends),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new ClassT(
			scope,
			originNode,
			{
				_constructor: data.constructor === undefined
					? undefined
					: getType(data.constructor),
				statics: Array(data.statics).map((id) => getType(id)),
				instances: Array(data.instances).map((id) => getType(id)),
				extends: data.extends === undefined ? undefined : getType(data.extends),
			},
		);
	}
}
