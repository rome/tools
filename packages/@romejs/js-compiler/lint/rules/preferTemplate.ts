/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from '@romejs/js-compiler';
import {
  AnyExpression,
  AnyNode,
  BinaryExpression,
  StringLiteral,
  TemplateLiteral,
  stringLiteral,
  templateElement,
  templateLiteral,
} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

// expr + expr
function isBinaryAddExpression(node: AnyNode): node is BinaryExpression {
  return node.type === 'BinaryExpression' && node.operator === '+';
}

// 'str' + 'str'
// 'str' + expr
// expr + 'str'
// expr + (expr + 'str')
// (expr + 'str') + expr
// expr * (expr + 'str')
// (expr * expr) + 'str'
function isUnnecessaryStringConcatExpression(
  node: AnyNode,
): node is BinaryExpression {
  if (node.type !== 'BinaryExpression') {
    return false;
  }

  if (node.left.type === 'BinaryExpression') {
    if (isUnnecessaryStringConcatExpression(node.left)) {
      return true;
    }
  }

  if (node.right.type === 'BinaryExpression') {
    if (isUnnecessaryStringConcatExpression(node.right)) {
      return true;
    }
  }

  if (!isBinaryAddExpression(node)) {
    return false;
  }

  if (node.left.type === 'StringLiteral' && !node.left.value.includes('`')) {
    return true;
  }

  if (node.right.type === 'StringLiteral' && !node.right.value.includes('`')) {
    return true;
  }

  return false;
}

// expr + expr + expr + ...
function collectBinaryAddExpressionExpressions(
  node: BinaryExpression,
): Array<AnyExpression> {
  let expressions: Array<AnyExpression> = [];

  if (isBinaryAddExpression(node.left)) {
    expressions = expressions.concat(
      collectBinaryAddExpressionExpressions(node.left),
    );
  } else {
    expressions.push(node.left);
  }

  if (isBinaryAddExpression(node.right)) {
    expressions = expressions.concat(
      collectBinaryAddExpressionExpressions(node.right),
    );
  } else {
    expressions.push(node.right);
  }

  return expressions;
}

// 'str' + 'str' + expr -> 'strstr' + expr
function reduceBinaryExpressionExpressions(expressions: Array<AnyExpression>) {
  let reducedExpressions: Array<AnyExpression> = [];
  let index = 0;

  while (index < expressions.length) {
    let current = expressions[index];

    if (current.type === 'StringLiteral') {
      let strings: Array<StringLiteral> = [current];

      while (index + 1 < expressions.length) {
        let next = expressions[index + 1];
        if (next.type === 'StringLiteral') {
          strings.push(next);
          index++;
        } else {
          break;
        }
      }

      if (strings.length === 1) {
        reducedExpressions.push(current);
      } else {
        reducedExpressions.push(
          stringLiteral.create({
            value: strings.map((string) => string.value).join(''),
          }),
        );
      }
    } else {
      reducedExpressions.push(current);
    }

    index++;
  }

  return reducedExpressions;
}

// 'str' + expr + 'str' -> `str${expr}str`
function convertExpressionsToTemplateLiteral(
  items: Array<AnyExpression>,
): TemplateLiteral {
  let expressions = [];
  let quasis = [];

  for (let index = 0; index < items.length; index++) {
    let item = items[index];
    let isTail = index === items.length - 1;
    let isHead = index === 0;

    if (item.type === 'StringLiteral') {
      quasis.push(
        templateElement.create({
          cooked: item.value,
          raw: item.value,
          tail: isTail,
        }),
      );
    } else {
      expressions.push(item);
      if (isTail || isHead) {
        quasis.push(
          templateElement.create({
            cooked: '',
            raw: '',
            tail: isTail,
          }),
        );
      }
    }
  }

  return templateLiteral.create({
    expressions,
    quasis,
  });
}

export default {
  name: 'preferTemplate',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (isUnnecessaryStringConcatExpression(node)) {
      let expressions = collectBinaryAddExpressionExpressions(node);
      let reducedExpressions = reduceBinaryExpressionExpressions(expressions);
      let template = convertExpressionsToTemplateLiteral(reducedExpressions);

      return path.context.addFixableDiagnostic(
        {
          old: node,
          fixed: template,
        },
        descriptions.LINT.PREFER_TEMPLATE,
      );
    }

    return node;
  },
};
