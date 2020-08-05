/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {getOptions, getPrefixedNamespace, getPrivateName} from "../_utils";
import {renameBindings} from "@internal/js-ast-utils";

export default createVisitor({
	name: "cjsRootTransform",
	enter(path) {
		const {node, scope, context} = path;

		const {moduleId} = getOptions(context);

		if (node.type === "JSRoot") {
			const mappings = new Map();

			// make all variables private
			for (const [name] of path.scope.getOwnBindings()) {
				mappings.set(name, getPrivateName(name, moduleId));
			}

			if (!scope.hasBinding("exports")) {
				mappings.set("exports", getPrefixedNamespace(moduleId));
			}

			const newProgram = renameBindings(path, mappings);
			return signals.replace(newProgram);
		}

		return signals.retain;
	},
});
