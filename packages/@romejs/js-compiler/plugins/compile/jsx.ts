/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  AnyExpression,
  ObjectProperty,
  JSXIdentifier,
  JSXNamespacedName,
  JSXAttribute,
  ObjectProperties,
  MemberExpression,
  ThisExpression,
  StringLiteral,
  JSXElement,
  staticPropertyKey,
  ReferenceIdentifier,
  referenceIdentifier,
  computedMemberProperty,
  staticMemberProperty,
  jsxNamespacedName,
  objectExpression,
  objectProperty,
  stringLiteral,
  identifier,
  memberExpression,
  thisExpression,
  booleanLiteral,
  nullLiteral,
  callExpression,
  jsxIdentifier,
  jsxElement,
  jsxExpressionContainer,
  computedPropertyKey,
  JSXExpressionContainer,
  CallExpression,
  spreadElement,
} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {
  template,
  isValidIdentifierName,
  inheritLoc,
} from '@romejs/js-ast-utils';

function isCompatTag(tagName: undefined | string): boolean {
  return tagName !== undefined ? /^[a-z]|-/.test(tagName) : false;
}

function convertJSXIdentifier(
  path: Path,
): MemberExpression | ThisExpression | StringLiteral | ReferenceIdentifier {
  const {node} = path;

  if (node.type === 'JSXReferenceIdentifier') {
    if (node.name === 'this') {
      return thisExpression.create({});
    } else if (isValidIdentifierName(node.name)) {
      return referenceIdentifier.create(
        {
          name: node.name,
        },
        node,
      );
    } else {
      return stringLiteral.quick(node.name);
    }
  } else if (node.type === 'JSXMemberExpression') {
    let prop = convertJSXIdentifier(path.getChildPath('property'));

    if (prop.type === 'ReferenceIdentifier') {
      return memberExpression.create({
        object: convertJSXIdentifier(path.getChildPath('object')),
        property: staticMemberProperty.quick(identifier.quick(prop.name)),
      });
    } else {
      return memberExpression.create({
        object: convertJSXIdentifier(path.getChildPath('object')),
        property: computedMemberProperty.quick(prop),
      });
    }
  } else {
    throw new Error(
      `Received a node of type ${node.type}, the only node types that should be in this position are JSXIdentifier and JSXMemberExpression`,
    );
  }
}

function convertAttributeValue(
  node: AnyExpression | JSXExpressionContainer,
): AnyExpression {
  if (node.type === 'JSXExpressionContainer') {
    return node.expression;
  } else {
    return node;
  }
}

function extractName(node: JSXIdentifier | JSXNamespacedName): string {
  if (node.type === 'JSXNamespacedName') {
    throw new Error('JSX is not XML blah blah blah');
  } else {
    return jsxIdentifier.assert(node).name;
  }
}

function convertAttribute(node: JSXAttribute): ObjectProperty {
  let valueNode = convertAttributeValue(
    node.value || booleanLiteral.create({value: true}),
  );
  if (
    valueNode.type === 'StringLiteral' &&
    (!node.value || node.value.type !== 'JSXExpressionContainer')
  ) {
    valueNode = stringLiteral.create({
      value: valueNode.value.replace(/\n\s+/g, ' '),
    });
  }

  const name = extractName(node.name);

  if (isValidIdentifierName(name)) {
    const nameNode = identifier.create({
      name,
      loc: inheritLoc(node),
    });

    return objectProperty.create({
      key: staticPropertyKey.quick(nameNode),
      value: valueNode,
    });
  } else {
    return objectProperty.create({
      key: computedPropertyKey.quick(stringLiteral.quick(name)),
      value: valueNode,
    });
  }
}

function pushProps(
  _props: ObjectProperties,
  objs: Array<AnyExpression>,
): ObjectProperties {
  if (!_props.length) {
    return _props;
  }

  objs.push(objectExpression.create({properties: _props}));
  return [];
}

function buildOpeningElementAttributes(attribs: JSXElement['attributes']) {
  let _props: ObjectProperties = [];
  const objs: Array<AnyExpression> = [];

  while (attribs.length > 0) {
    const prop = attribs.shift();
    if (prop === undefined) {
      throw new Error('Already validated length');
    }

    if (prop.type === 'JSXSpreadAttribute') {
      _props = pushProps(_props, objs);
      objs.push(prop.argument);
    } else {
      _props.push(convertAttribute(prop));
    }
  }

  pushProps(_props, objs);

  let ret: AnyExpression;
  if (objs.length === 1) {
    // only one object
    ret = objs[0];
  } else {
    // looks like we have multiple objects
    if (objs[0].type !== 'ObjectExpression') {
      objs.unshift(objectExpression.create({properties: []}));
    }

    // spread it
    ret = callExpression.create({
      callee: template.expression`Object.assign`,
      arguments: objs,
    });
  }

  return ret;
}

function cleanJSXElementLiteralChild(value: string): undefined | StringLiteral {
  const lines = value.split(/\r\n|\n|\r/);

  let lastNonEmptyLine = 0;

  for (let i = 0; i < lines.length; i++) {
    if (lines[i].match(/[^ \t]/)) {
      lastNonEmptyLine = i;
    }
  }

  let str = '';

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    const isFirstLine = i === 0;
    const isLastLine = i === lines.length - 1;
    const isLastNonEmptyLine = i === lastNonEmptyLine;

    // replace rendered whitespace tabs with spaces
    let trimmedLine = line.replace(/\t/g, ' ');

    // trim whitespace touching a newline
    if (!isFirstLine) {
      trimmedLine = trimmedLine.replace(/^[ ]+/, '');
    }

    // trim whitespace touching an endline
    if (!isLastLine) {
      trimmedLine = trimmedLine.replace(/[ ]+$/, '');
    }

    if (trimmedLine) {
      if (!isLastNonEmptyLine) {
        trimmedLine += ' ';
      }

      str += trimmedLine;
    }
  }

  if (str != '') {
    return stringLiteral.quick(str);
  }
}

function buildChildren(
  children: JSXElement['children'],
): CallExpression['arguments'] {
  const elems: CallExpression['arguments'] = [];

  for (let child of children) {
    if (child.type === 'JSXText') {
      const node = cleanJSXElementLiteralChild(child.value);
      if (node !== undefined) {
        elems.push(node);
      }
      continue;
    }

    if (child.type === 'JSXExpressionContainer') {
      const {expression} = child;
      if (expression.type !== 'JSXEmptyExpression') {
        elems.push(child.expression);
      }
      continue;
    }

    if (child.type === 'JSXSpreadChild') {
      elems.push(spreadElement.quick(child.expression));
      continue;
    }

    elems.push(child);
  }

  return elems;
}

export default {
  name: 'jsx',
  enter(path: Path): AnyNode {
    const {node, context, parent} = path;

    if (jsxElement.is(node)) {
      let type = convertJSXIdentifier(path.getChildPath('name'));

      if (jsxNamespacedName.is(node.name)) {
        // TODO better handle this
        context.addNodeDiagnostic(type, {
          category: 'compile/jsx',
          message: 'JSX is not XML',
        });
      }

      if (type.type === 'ReferenceIdentifier' && isCompatTag(type.name)) {
        type = stringLiteral.quick(type.name);
      }

      let attribs: AnyExpression;
      if (node.attributes.length > 0) {
        attribs = buildOpeningElementAttributes(node.attributes);
      } else {
        attribs = nullLiteral.create({});
      }

      const call = callExpression.create({
        callee: template.expression`React.createElement`,
        arguments: [type, attribs, ...buildChildren(node.children)],
      });

      // If we're a JSX element child then we need to be wrapped
      if (jsxElement.is(parent)) {
        return jsxExpressionContainer.create({
          expression: call,
        });
      } else {
        return call;
      }
    }

    if (node.type === 'JSXFragment') {
      const type = template.expression`React.Fragment`;
      const attribs = template.expression`null`;
      return callExpression.create({
        callee: template.expression`React.createElement`,
        arguments: [type, attribs, ...buildChildren(node.children)],
      });
    }

    return node;
  },
};
