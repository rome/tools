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
	jsStringLiteral,
} from "@internal/ast";
import {template} from "@internal/js-ast-utils";
import {CompilerContext, createVisitor, signals} from "@internal/compiler";

function isImportMeta(node: AnyNode): node is JSMetaProperty {
	return (
		node.type === "JSMetaProperty" &&
		node.meta.name === "import" &&
		node.property.name === "meta"
	);
}

function createURLString(context: CompilerContext): AnyJSExpression {
	const str = jsStringLiteral.create({
		value: `file://${getFilename(context)}`,
	});
	return template.expression`typeof __filename === 'string' ? 'file://' + __filename : ${str}`;
}

function getFilename(context: CompilerContext): string {
	const {path} = context;
	if (path === undefined) {
		return "";
	} else {
		return path.join();
	}
}

export default createVisitor({
	name: "jsMetaPropertyTransform",
	enter(path) {
		const {node, context} = path;

		// Inline __filenamd and __dirname

		/*if (
      node.type === 'ReferenceIdentifier' &&
      (node.type === '__dirname' || node.name === '__filename')
    ) {
      if (node.type === '__dirname') {
        return jsStringLiteral.create({
          value: pathUtils.dirname(getFilename(context)),
        });
      }

      if (node.type === '__filename') {
        return jsStringLiteral.create({
          value: getFilename(context),
        });
      }
    }*/

		// Direct reference to import.meta.url
		if (
			node.type === "JSMemberExpression" &&
			node.property.type === "JSStaticMemberProperty" &&
			isImportMeta(node.object) &&
			node.property.value.type === "JSIdentifier" &&
			node.property.value.name === "url"
		) {
			return signals.replace(createURLString(context));
		}

		// This is an escaped import.meta or else our other transform would have changed it
		if (isImportMeta(node)) {
			return signals.replace(
				template.expression`({url: ${createURLString(context)}})`,
			);
		}

		return signals.retain;
	},
});
