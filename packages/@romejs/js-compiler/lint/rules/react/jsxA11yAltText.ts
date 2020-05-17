/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {JSXElement, AnyNode} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

function hasUndefinedAltValue(node: AnyNode): boolean {
  if (node.type !== "JSXExpressionContainer") {
    return false;
  }
  return (
    (node.expression.type === "ReferenceIdentifier" &&
      node.expression.name === "undefined") ||
    (node.expression.type === "TemplateLiteral" &&
      node.expression.expressions.some(
        expression =>
          expression.type === "ReferenceIdentifier" &&
          expression.name === "undefined"
      ))
  );
}

function hasImgAltText(node: JSXElement): boolean {
  return node.attributes.some(
    attr =>
      attr.type === "JSXAttribute" &&
      attr.name.name === "alt" &&
      attr.value &&
      !hasUndefinedAltValue(attr.value)
  );
}

function hasObjectAltText(node: JSXElement): boolean {
  return (
    node.attributes.some(
      attr =>
        attr.type === "JSXAttribute" && attr.name.type === "JSXIdentifier" &&
        /(aria-label)|(aria-labelledby)|(title)/.test(attr.name.name) && 
        attr.value &&
        !hasUndefinedAltValue(attr.value)
    ) || node.children.length > 0
  );
}

function hasAreaAltText(node: JSXElement): boolean {
  return node.attributes.some(
    attr =>
      attr.type === "JSXAttribute" && attr.name.type === "JSXIdentifier" &&
      /(aria-label)|(aria-labelledby)|(alt)|(title)/.test(attr.name.name) &&
      attr.value &&
      !hasUndefinedAltValue(attr.value)
  );
}

function hasInputAltText(node: JSXElement): boolean {
  return node.attributes.some(
    attr =>
      attr.type === "JSXAttribute" && attr.name.type === "JSXIdentifier" &&
      /(aria-label)|(aria-labelledby)|(alt)|(title)/.test(attr.name.name) &&
      attr.value && 
      !hasUndefinedAltValue(attr.value)
  );
}

function hasTypeImage(node: JSXElement): boolean {
  return node.attributes.some(
    attr =>
      attr.type === "JSXAttribute" &&
      attr.name.name === "type" &&
      attr.value &&
      attr.value.type === "StringLiteral" &&
      attr.value.value === "image"
  );
}

export default {
  name: "jsxA11yAltText",
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (node.type === "JSXElement" && node.name.type === "JSXIdentifier") {
      if (!/(img)|(area)|(input)|(object)/.test(node.name.name)) {
        return node;
      }

      if (
        (node.name.name === "img" && !hasImgAltText(node)) ||
        (node.name.name === "object" && !hasObjectAltText(node)) ||
        (node.name.name === "area" && !hasAreaAltText(node)) ||
        (node.name.name === "input" &&
          hasTypeImage(node) &&
          !hasInputAltText(node))
      ) {
        path.context.addNodeDiagnostic(
          node,
          descriptions.LINT.JSX_A11Y_ALT_TEXT
        );
      }
    }

    return node;
  }
};
