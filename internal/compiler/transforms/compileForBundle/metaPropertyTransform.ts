/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyNode,
	JSMetaProperty,
	JSReferenceIdentifier,
} from "@internal/ast";
import {template} from "@internal/js-ast-utils";
import {createVisitor, signals} from "@internal/compiler";
import {getOptions} from "./_utils";

function isImportMeta(node: AnyNode): node is JSMetaProperty {
	return (
		node.type === "JSMetaProperty" &&
		node.meta.name === "import" &&
		node.property.name === "meta"
	);
}

function createURLString(): AnyJSExpression {
	return template.expression`"file://" + __filename`;
}

const BLESSED_DIRNAME: JSReferenceIdentifier = {
	type: "JSReferenceIdentifier",
	name: "__dirname",
};

export default createVisitor({
	name: "jsMetaPropertyTransform",
	enter(path) {
		const {node, context} = path;
		const options = getOptions(context);

		// Inline __dirname
		if (
			node.type === "JSReferenceIdentifier" &&
			node.name === "__dirname" &&
			node !== BLESSED_DIRNAME
		) {
			return signals.replace(
				template.expression`${BLESSED_DIRNAME} + '/' + "${options.__filename.getParent().join()}"`,
			);
		}

		// Inline __filename
		if (node.type === "JSReferenceIdentifier" && node.name === "__filename") {
			return signals.replace(
				template.expression`${BLESSED_DIRNAME} + '/' + "${options.__filename.join()}"`,
			);
		}

		// Direct reference to import.meta.url
		if (
			node.type === "JSMemberExpression" &&
			node.property.type === "JSStaticMemberProperty" &&
			isImportMeta(node.object) &&
			node.property.value.type === "JSIdentifier" &&
			node.property.value.name === "url"
		) {
			return signals.replace(createURLString());
		}

		// This is an escaped import.meta or else our other transform would have changed it
		if (isImportMeta(node)) {
			return signals.replace(template.expression`({url: ${createURLString()}})`);
		}

		return signals.retain;
	},
});
