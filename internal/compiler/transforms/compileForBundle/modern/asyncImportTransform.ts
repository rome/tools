/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {template} from "@internal/js-ast-utils";
import {jsStringLiteral} from "@internal/ast";
import {getModuleId, getOptions} from "../_utils";

export default createVisitor({
	name: "asyncImportTransform",
	enter(path) {
		const {node, context} = path;
		const opts = getOptions(context);

		// desugar import('source') to Rome.import(moduleId)
		if (node.type === "JSImportCall" && node.argument.type === "JSStringLiteral") {
			const moduleId = getModuleId(node.argument.value, opts);
			if (moduleId !== undefined) {
				const id = jsStringLiteral.create({
					loc: node.argument.loc,
					value: moduleId,
				});
				return signals.replace(template.expression`Rome.import(${id})`);
			}
		}

		return signals.retain;
	},
});
