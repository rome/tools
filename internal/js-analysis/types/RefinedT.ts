/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import MissingUnionE from "./errors/MissingUnionE";
import T from "./T";

export default class RefinedT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, root: T, remove: T) {
		super(scope, originNode);
		this.root = root;
		this.remove = remove;
	}

	public static type = "RefinedT";
	private root: T;
	private remove: T;

	public reduce(): T {
		const {root} = this;

		const exploded = this.utils.explodeUnion(root);
		const removeTypes = this.utils.explodeUnion(this.remove);

		const clean = [];
		const removed = [];

		// remove any possible derived types from the root that are compatible with the removed type
		for (const type of exploded) {
			let compatible = false;

			// check if any of the removed types are compatible, if every removed type is incompatible then

			// we've refined away the type
			for (const remove of removeTypes) {
				if (this.utils.isCompatibleWith(type, remove)) {
					compatible = true;
				}
			}

			if (compatible) {
				removed.push(type);
			} else {
				clean.push(type);
			}
		}

		if (removed.length === 0) {
			// return an error here because the removed type doesn't exist in the root
			return new MissingUnionE(
				root.scope,
				root.originNode,
				root,
				this.remove,
				removed,
			);
		} else {
			return root.scope.createUnion(clean, root.originNode);
		}
	}
}
