/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {HydrateData, HydrateTypeFactory} from "../Evaluator";
import T, {SerialTypeFactory, TypeCompatibilityReturn} from "./T";
import {Scope} from "../scopes";
import {HumanBuilder} from "../Utils";
import ObjPropT from "./ObjPropT";

export default class ObjT extends T {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		opts: {
			props?: Array<T>;
			proto: undefined | T;
			calls?: Array<T>;
		},
	) {
		super(scope, originNode);
		this.calls = opts.calls === undefined ? [] : opts.calls;
		this.props = opts.props === undefined ? [] : opts.props;
		this.proto = opts.proto;
	}

	static type = "ObjT";
	calls: Array<T>;
	props: Array<T>;
	proto: undefined | T;

	serialize(addType: SerialTypeFactory): HydrateData {
		if (this.constructor !== ObjT) {
			throw new Error(
				"Expected ObjT to be constructor, youve likely forgot to define this method in the type subclass",
			);
		}

		return {
			calls: this.calls.map((type) => addType(type)),
			proto: this.proto === undefined ? undefined : addType(this.proto),
			props: this.props.map((type) => addType(type)),
		};
	}

	static hydrate(
		scope: Scope,
		originNode: AnyNode,
		data: HydrateData,
		getType: HydrateTypeFactory,
	): T {
		return new ObjT(
			scope,
			originNode,
			{
				props: Array(data.props).map((id) => getType(id)),
				proto: data.proto === undefined ? undefined : getType(data.proto),
				calls: Array(data.calls).map((id) => getType(id)),
			},
		);
	}

	compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
		if (!(otherType instanceof ObjT)) {
			return false;
		}

		const ourProps: Array<T> = this.props;
		const theirProps: Array<T> = otherType.props;

		// check that the other type has all of our props
		for (const ourPropRaw of ourProps) {
			// reduce and get the key of this prop
			const ourProp = this.utils.reduce(ourPropRaw);
			let key;
			if (ourProp instanceof ObjPropT) {
				key = ourProp.key;
			} else {
				// should probably do something here
				continue;
			}

			// try and find a prop of the same key in the other object
			let theirProp;
			for (const theirPropRaw of theirProps) {
				const maybeTheirProp = this.utils.reduce(theirPropRaw);
				if (maybeTheirProp instanceof ObjPropT && maybeTheirProp.key === key) {
					theirProp = maybeTheirProp;
					break;
				}
			}

			if (!ourProp || !theirProp) {
				return false;
			}

			const compatibility = this.utils.checkCompability(ourProp, theirProp);
			if (compatibility.type === "incompatible") {
				return compatibility;
			}
		}

		return true;
	}

	humanize(builder: HumanBuilder): string {
		if (this.props.length === 0) {
			return "{}";
		} else {
			return [
				"{",
				...this.props.map((prop) => {
					const val = builder.humanize(prop);
					let lines = val.split("\n");
					lines = lines.map((line) => `  ${line}`);
					return `${lines.join("\n")},`;
				}),
				"}",
			].join("\n");
		}
	}
}
