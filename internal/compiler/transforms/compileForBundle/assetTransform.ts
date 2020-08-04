/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {getOptions} from "./_utils";
import {ASSET_EXPORT_TEMPORARY_VALUE} from "@internal/core/common/file-handlers";

export default createVisitor({
	name: "asset",
	enter(path) {
		const {node} = path;
		const options = getOptions(path.context);

		if (
			node.type === "JSExportDefaultDeclaration" &&
			node.declaration.type === "JSStringLiteral" &&
			node.declaration.value === ASSET_EXPORT_TEMPORARY_VALUE &&
			options.assetPath !== undefined
		) {
			return signals.replace({
				...node,
				declaration: {
					...node.declaration,
					value: options.moduleId,
				},
			});
		}

		return signals.retain;
	},
});
