/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {createPath} from "@internal/path";
import {filenameToId} from "./useDefaultExportBasename";

export default createLintVisitor({
	name: "js/useDefaultImportBasename",
	enter(path) {
		const {node} = path;

		if (node.type === "JSImportDeclaration") {
			const {defaultSpecifier} = node;
			if (defaultSpecifier === undefined) {
				return signals.retain;
			}

			const filePath = createPath(node.source.value);
			if (!filePath.isExplicitRelative()) {
				return signals.retain;
			}

			const expectedName = filenameToId(filePath, false);
			const expectedNameCapital = filenameToId(filePath, true);
			if (expectedName === undefined || expectedNameCapital === undefined) {
				return signals.retain;
			}

			const localName = defaultSpecifier.local.name.name;
			if (localName !== expectedName && localName !== expectedNameCapital) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_USE_DEFAULT_IMPORT_BASENAME(
						localName,
						[expectedName, expectedNameCapital],
					),
				);
			}
		}

		return signals.retain;
	},
});
