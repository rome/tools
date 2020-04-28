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
  TemplateElement,
  TemplateLiteral,
  stringLiteral,
  templateElement,
  templateLiteral,
} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

type TemplatePart = AnyExpression | TemplateElement;
type StaticString = StringLiteral | TemplateElement;

// expr + expr
function isBinaryAddExpression(node: AnyNode): node is BinaryExpression {
  return node.type === 'BinaryExpression' && node.operator === '+';
}

// 'str' + 'str'
// 'str' + expr
// expr + 'str'
// expr + (expr + 'str')
// (expr + 'str') + expr
function isUnnecessaryStringConcatExpression(
  node: AnyNode,
): node is BinaryExpression {
  if (!isBinaryAddExpression(node)) {
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

  if (node.left.type === 'StringLiteral' && !node.left.value.includes('`')) {
    return true;
  }

  if (node.right.type === 'StringLiteral' && !node.right.value.includes('`')) {
    return true;
  }

  if (node.left.type === 'TemplateLiteral') {
    return true;
  }

  if (node.right.type === 'TemplateLiteral') {
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

// zips template.quasis and template.expressions into one array
function zipTemplateLiteralParts(template: TemplateLiteral): Array<TemplatePart> {
  let templateParts = [];

  for (let i = 0; i < template.quasis.length; i++) {
    templateParts.push(template.quasis[i]);

    if (i + 1 < template.quasis.length) {
      templateParts.push(template.expressions[i]);
    }
  }

  return templateParts;
}

// flattens an array of expressions into TemplateLiteral parts
function flattenExpressionsToTemplateParts(
  expressions: Array<AnyExpression>,
): Array<TemplatePart> {
  let parts: Array<TemplatePart> = [];
  let queue: Array<TemplatePart> = [...expressions];

  while (true) {
    let node = queue.shift();
    if (!node) break;

    if (node.type === 'TemplateLiteral') {
      queue = [...zipTemplateLiteralParts(node), ...queue];
    } else {
      parts.push(node);
    }
  }

  return parts;
}

// 'str' + 'str' + expr -> 'strstr' + expr
function combineTemplateParts(expressions: Array<TemplatePart>) {
  let reducedExpressions: Array<AnyExpression> = [];
  let index = 0;

  while (index < expressions.length) {
    let current = expressions[index];

    if (current.type === 'StringLiteral' || current.type === 'TemplateElement') {
      let strings: Array<StaticString> = [current];

      while (index + 1 < expressions.length) {
        let next = expressions[index + 1];
        if (next.type === 'StringLiteral' || next.type === 'TemplateElement') {
          strings.push(next);
          index++;
        } else {
          break;
        }
      }

      if (strings.length === 1 && current.type === 'StringLiteral') {
        reducedExpressions.push(current);
      } else {
        reducedExpressions.push(
          stringLiteral.create({
            value: strings.map((string) => {
              if (string.type === 'TemplateElement') {
                return string.raw;
              } else {
                return string.value;
              }
            }).join(''),
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
function convertTemplatePartsToTemplateLiteral(
  nodes: Array<TemplatePart>,
): TemplateLiteral {
  let templateExpressions: Array<AnyExpression> = [];
  let templateQuasis: Array<TemplateElement> = [];

  for (let index = 0; index < nodes.length; index++) {
    let node = nodes[index];
    let isTail = index === nodes.length - 1;
    let isHead = index === 0;

    if (node.type === 'TemplateElement') {
      templateElement;
    } else if (node.type === 'StringLiteral') {
      templateQuasis.push(
        templateElement.create({
          cooked: node.value,
          raw: node.value,
          tail: isTail,
        }),
      );
    } else {
      templateExpressions.push(node);
      if (isTail || isHead) {
        templateQuasis.push(
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
    expressions: templateExpressions,
    quasis: templateQuasis,
  });
}

// Ignore:
// str + str
// str + str + str
// Replace:
// str + expr
// str + expr + str
function shouldReplace(expressions: Array<AnyExpression>): boolean {
  for (let expression of expressions) {
    if (expression.type !== 'StringLiteral') {
      return true;
    }
  }

  return false;
}

export default {
  name: 'preferTemplate',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (isUnnecessaryStringConcatExpression(node)) {
      let expressions = collectBinaryAddExpressionExpressions(node);

      if (shouldReplace(expressions)) {
        let templateParts = flattenExpressionsToTemplateParts(expressions);
        let combinedParts = combineTemplateParts(templateParts);
        let template = convertTemplatePartsToTemplateLiteral(combinedParts);

        return path.context.addFixableDiagnostic(
          {
            old: node,
            fixed: template,
          },
          descriptions.LINT.PREFER_TEMPLATE,
        );
      }
    }

    return node;
  },
};
