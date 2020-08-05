/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "../scopes";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class BlockT extends T {
	constructor(scope: Scope, originNode: undefined | AnyNode, body: Array<T>) {
		super(scope, originNode);
		this.body = body;
	}

	public static type = "BlockT";

	private body: Array<T>;

	public reduce(): T {
		const body = [];
		let changed = false;

		for (const type of this.body) {
			const reduced = this.utils.reduce(type);
			body.push(reduced);
			if (reduced !== type) {
				changed = true;
			}
		}

		if (changed) {
			return new BlockT(this.scope, this.originNode, body);
		} else {
			return this;
		}
	}

	public humanize(): StaticMarkup {
		return markup`{}`;
	}
}
