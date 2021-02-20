/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import {ExpressionNode} from "@internal/codec-spdx-license/types";

export function stringifySPDXLicense(node: ExpressionNode): string {
	// TODO parens
	switch (node.type) {
		case "Or":
			return `${stringifySPDXLicense(node.left)} OR ${stringifySPDXLicense(
				node.right,
			)}`;

		case "And":
			return `${stringifySPDXLicense(node.left)} AND ${stringifySPDXLicense(
				node.right,
			)}`;

		case "License": {
			let str = node.id;
			if (node.plus) {
				str += "+";
			}
			if (node.exception !== undefined) {
				str += ` WITH ${node.exception}`;
			}
			return str;
		}
	}
}
