/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import { Path } from "@romejs/js-compiler";
import { AnyNode, ObjectProperty, AnyClassMember } from "@romejs/js-ast";
import { descriptions } from "@romejs/diagnostics";

function hasComponentDidUpdate(node: ObjectProperty | AnyClassMember): boolean {
  return (
    node.key.type === "StaticPropertyKey" &&
    node.key.value.type === "Identifier" &&
    node.key.value.name === "componentDidUpdate"
  );
}

function hasSetState(node: AnyNode): boolean {
  if (
    node.type === "ObjectProperty" &&
    node.value.type === "FunctionExpression"
  ) {
    if (node.value.body.type === "BlockStatement") {
      return node.value.body.body.some(
        statement =>
          statement.type === "ExpressionStatement" &&
          statement.expression.type === "CallExpression" &&
          statement.expression.callee.type === "MemberExpression" &&
          statement.expression.callee.property.value.type === "Identifier" &&
          statement.expression.callee.property.value.name === "setState"
      );
    }
  }
  if (node.type === "ClassMethod") {
    if (node.body.type === "BlockStatement") {
      return node.body.body.some(
        statement =>
          statement.type === "ExpressionStatement" &&
          statement.expression.type === "CallExpression" &&
          statement.expression.callee.type === "MemberExpression" &&
          statement.expression.callee.property.value.type === "Identifier" &&
          statement.expression.callee.property.value.name === "setState"
      );
    }
  }
  return false;
}

function hasSetStateInComponentDidUpdate(node: AnyNode): boolean {
  if (node.type === "ObjectExpression") {
    return node.properties.some(
      prop =>
        prop.type === "ObjectProperty" &&
        hasComponentDidUpdate(prop) &&
        hasSetState(prop)
    );
  }

  if (node.type === "ClassDeclaration") {
    return node.meta.body.some(
      method => hasComponentDidUpdate(method) && hasSetState(method)
    );
  }
  return false;
}

export default {
  name: "noDidUpdateSetState",
  enter(path: Path): AnyNode {
    const { node } = path;

    if (hasSetStateInComponentDidUpdate(node)) {
      path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.NO_DID_UPDATE_SET_STATE
      );
    }
    return node;
  }
};
