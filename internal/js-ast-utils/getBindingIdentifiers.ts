/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSBindingIdentifier, bindingKeys} from "@internal/ast";

export function getBindingIdentifiers(
	node: AnyNode | Array<AnyNode>,
): Array<JSBindingIdentifier> {
	const ids: Array<JSBindingIdentifier> = [];
	let queue: Array<undefined | AnyNode> = Array.isArray(node)
		? [...node]
		: [node];

	while (queue.length) {
		const node = queue.pop();
		if (node === undefined) {
			continue;
		}

		if (node.type === "JSBindingIdentifier") {
			ids.push(node);
			continue;
		}

		const keys: undefined | Array<string> = bindingKeys.get(node.type);
		if (keys === undefined) {
			continue;
		}

		for (const key of keys) {
			// rome-ignore lint/ts/noExplicitAny
			const val = (node as any)[key];
			if (val === undefined) {
				continue;
			} else if (Array.isArray(val)) {
				queue = queue.concat(val);
			} else {
				queue.push(val);
			}
		}
	}

	return ids;
}
