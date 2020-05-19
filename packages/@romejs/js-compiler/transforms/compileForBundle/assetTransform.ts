/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {getOptions} from "./_utils";
import {ASSET_EXPORT_TEMPORARY_VALUE} from "@romejs/core/common/file-handlers/index";

export default {
	name: "asset",
	enter(path: Path) {
		const {node} = path;
		const options = getOptions(path.context);

		if (
			node.type === "JSExportDefaultDeclaration" &&
			node.declaration.type === "JSStringLiteral" &&
			node.declaration.value === ASSET_EXPORT_TEMPORARY_VALUE &&
			options.assetPath !== undefined
		) {
			return {
				...node,
				declaration: {
					...node.declaration,
					value: options.moduleId,
				},
			};
		}

		return node;
	},
};
