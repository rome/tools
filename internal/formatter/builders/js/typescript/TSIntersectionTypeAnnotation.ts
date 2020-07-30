/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSIntersectionTypeAnnotation} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	indent,
	lineOrSpace,
	space,
} from "@internal/formatter";

export default function TSIntersectionTypeAnnotation(
	builder: Builder,
	node: TSIntersectionTypeAnnotation,
): Token {
	const parts = [];
	let shouldIndent = false;
	let previous: undefined | AnyNode;

	for (let i = 0; i < node.types.length; i++) {
		const type = node.types[i];
		const printed = builder.tokenize(type, node);

		if (previous === undefined) {
			parts.push(printed);
		} else if (isObjectType(previous) && isObjectType(type)) {
			//   let foo: {
			//     a: string;
			//     b: string;
			//   } & {
			//     c: string;
			//     d: string;
			//   };
			parts.push(space, "&", space, shouldIndent ? indent(printed) : printed);
		} else if (!isObjectType(previous) && !isObjectType(type)) {
			//   let foo: XXXX &
			//     YYYY &&
			//     ZZZZ;
			parts.push(indent(concat([space, "&", lineOrSpace, printed])));
		} else {
			//   let z: AAA & {
			//     a: string;
			//     b: string;
			//   } & BBB &
			//     CCC & {
			//       c: string;
			//       d: string;
			//     };
			if (i > 1) {
				shouldIndent = true;
			}

			parts.push(space, "&", space, shouldIndent ? indent(printed) : printed);
		}

		previous = type;
	}

	return group(concat(parts));
}

function isObjectType(node: AnyNode): boolean {
	return node.type === "TSMappedType" || node.type === "TSObjectTypeAnnotation";
}
