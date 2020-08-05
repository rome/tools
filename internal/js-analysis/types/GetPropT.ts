/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory} from "./T";
import ObjPropT from "./ObjPropT";
import UnknownPropE from "./errors/UnknownPropE";
import ObjIndexPropT from "./ObjIndexPropT";
import StringLiteralT from "./StringLiteralT";
import UnknownT from "./UnknownT";
import AnyT from "./AnyT";
import ObjT from "./ObjT";
import E from "./errors/E";

export default class GetPropT extends T {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		object: T,
		property: T,
	) {
		super(scope, originNode);
		this.object = object;
		this.property = property;
	}

	public static type = "GetPropT";

	private object: T;
	private property: T;

	public serialize(addType: SerialTypeFactory): HydrateData {
		return {
			object: addType(this.object),
			property: addType(this.property),
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new GetPropT(
			scope,
			originNode,
			getType(data.object),
			getType(data.property),
		);
	}

	private lookup(
		object: T,
		property: T,
		opts: {
			topObject?: T;
			protoKeys?: Array<string>;
		} = {},
	): T {
		object = this.utils.reduce(object);
		property = this.utils.reduce(property);

		const thisKeys: Set<string> = new Set();

		//
		const protoKeys = opts.protoKeys === undefined ? [] : opts.protoKeys;
		const topObject = opts.topObject === undefined ? object : opts.topObject;

		// turn property into string key
		let key: undefined | string;
		if (property instanceof StringLiteralT) {
			key = property.value;
		}

		// look up on object
		if (key !== undefined && object instanceof ObjT) {
			//
			const indexers: Array<ObjIndexPropT> = [];
			for (const maybePropRaw of object.props) {
				const maybeProp = this.utils.reduce(maybePropRaw);
				if (maybeProp instanceof ObjPropT) {
					if (maybeProp.key === key) {
						// TODO collate these in case there's multiple properties of this name
						return this.utils.reduce(maybeProp.value);
					} else {
						thisKeys.add(maybeProp.key);
					}
				} else if (maybeProp instanceof ObjIndexPropT) {
					indexers.push(maybeProp);
				}
			}

			//
			for (const indexer of indexers) {
				if (this.utils.isCompatibleWith(indexer.key, property)) {
					return this.utils.reduce(indexer.value);
				}
			}

			//
			if (object.proto) {
				return this.lookup(
					object.proto,
					property,
					{
						topObject,
						protoKeys: [...protoKeys, ...thisKeys],
					},
				);
			}
		}

		// property lookups on an `any` return `any`!
		if (object instanceof AnyT || object instanceof E) {
			return new AnyT(this.scope, this.originNode);
		}

		//
		if (typeof key === "string") {
			return new UnknownPropE(
				this.scope,
				this.originNode,
				{
					object: topObject,
					property,
					key,
					thisKeys: Array.from(thisKeys),
					protoKeys,
				},
			);
		} else {
			return new UnknownT(this.scope, this.originNode);
		}
	}

	public reduce(): T {
		return this.lookup(this.object, this.property);
	}
}
