/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {} from '@romejs/js-ast';
import {
  isFor,
  isUnaryLike,
  isConditional,
  isBinary,
} from '@romejs/js-ast-utils';
import {
  AnyNode,
  FlowNullableTypeAnnotation,
  UpdateExpression,
  ObjectExpression,
  DoExpression,
  LogicalExpression,
  BinaryExpression,
  SequenceExpression,
  YieldExpression,
  ClassExpression,
  UnaryExpression,
  SpreadElement,
  SpreadProperty,
  ArrowFunctionExpression,
  AssignmentExpression,
  ConditionalExpression,
  UnionTypeAnnotation,
  FlowFunctionTypeAnnotation,
} from '@romejs/js-ast';

const PRECEDENCE = {
  '||': 0,
  '&&': 1,
  '??': 1,
  '|': 2,
  '^': 3,
  '&': 4,
  '==': 5,
  '===': 5,
  '!=': 5,
  '!==': 5,
  '<': 6,
  '>': 6,
  '<=': 6,
  '>=': 6,
  in: 6,
  instanceof: 6,
  '>>': 7,
  '<<': 7,
  '>>>': 7,
  '+': 8,
  '-': 8,
  '*': 9,
  '/': 9,
  '%': 9,
  '**': 10,
};

const parens: Map<
  string,
  (node: any, parent: AnyNode, printStack: Array<AnyNode>) => boolean
> = new Map();
export default parens;

parens.set(
  'FlowNullableTypeAnnotation',
  (node: FlowNullableTypeAnnotation, parent: AnyNode): boolean => {
    return parent.type === 'FlowArrayTypeAnnotation';
  },
);

parens.set('UpdateExpression', function UpdateExpression(
  node: UpdateExpression,
  parent: AnyNode,
): boolean {
  // (foo++).test()
  return parent.type === 'MemberExpression' && parent.object === node;
});

parens.set('ObjectExpression', function ObjectExpression(
  node: ObjectExpression,
  parent: AnyNode,
  printStack: Array<AnyNode>,
): boolean {
  return isFirstInStatement(printStack, true, false);
});

parens.set('DoExpression', function DoExpression(
  node: DoExpression,
  parent: AnyNode,
  printStack: Array<AnyNode>,
): boolean {
  return isFirstInStatement(printStack, false, false);
});

function LogicalExpression(
  node: LogicalExpression | BinaryExpression,
  parent: AnyNode,
): boolean {
  if (
    node.operator === '**' &&
    parent.type === 'BinaryExpression' &&
    parent.operator === '**'
  ) {
    return parent.left === node;
  }

  if (
    ((parent.type === 'CallExpression' || parent.type === 'NewExpression') &&
      parent.callee === node) ||
    isUnaryLike(parent) ||
    (parent.type === 'MemberExpression' && parent.object === node) ||
    parent.type === 'AwaitExpression'
  ) {
    return true;
  }

  if (isBinary(parent)) {
    const parentOp = parent.operator;
    // @ts-ignore
    const parentPos = PRECEDENCE[parentOp];

    const nodeOp = node.operator;
    const nodePos = PRECEDENCE[nodeOp];

    if (
      // Logical expressions with the same precedence don't need parens.
      (parentPos === nodePos &&
        parent.right === node &&
        parent.type !== 'LogicalExpression') ||
      parentPos > nodePos
    ) {
      return true;
    }
  }

  return false;
}

parens.set('LogicalExpression', LogicalExpression);

parens.set('BinaryExpression', function BinaryExpression(
  node: BinaryExpression,
  parent: AnyNode,
): boolean {
  // let i = (1 in []);
  // for ((1 in []);;);
  return (
    (node.operator === 'in' &&
      (parent.type === 'VariableDeclarator' || isFor(parent))) ||
    LogicalExpression(node, parent)
  );
});

parens.set('SequenceExpression', function SequenceExpression(
  node: SequenceExpression,
  parent: AnyNode,
): boolean {
  if (
    // Although parentheses wouldn"t hurt around sequence
    // expressions in the head of for loops, traditional style
    // dictates that e.g. i++, j++ should not be wrapped with
    // parentheses.
    parent.type === 'ForStatement' ||
    parent.type === 'ThrowStatement' ||
    parent.type === 'ReturnStatement' ||
    (parent.type === 'IfStatement' && parent.test === node) ||
    (parent.type === 'WhileStatement' && parent.test === node) ||
    (parent.type === 'ForInStatement' && parent.right === node) ||
    (parent.type === 'SwitchStatement' && parent.discriminant === node) ||
    (parent.type === 'ExpressionStatement' && parent.expression === node)
  ) {
    return false;
  }

  // Otherwise err on the side of overparenthesization, adding
  // explicit exceptions above if this proves overzealous.
  return true;
});

function YieldExpression(node: YieldExpression, parent: AnyNode): boolean {
  return (
    isBinary(parent) ||
    isUnaryLike(parent) ||
    parent.type === 'CallExpression' ||
    parent.type === 'MemberExpression' ||
    parent.type === 'NewExpression' ||
    (parent.type === 'ConditionalExpression' && node === parent.test)
  );
}

parens.set('YieldExpression', YieldExpression);
parens.set('AwaitExpression', YieldExpression);

parens.set('ClassExpression', function ClassExpression(
  node: ClassExpression,
  parent: AnyNode,
  printStack: Array<AnyNode>,
): boolean {
  return isFirstInStatement(printStack, false, true);
});

function UnaryExpression(
  node:
    | UnaryExpression
    | ArrowFunctionExpression
    | AssignmentExpression
    | ConditionalExpression
    | SpreadElement
    | SpreadProperty,
  parent: AnyNode,
): boolean {
  return (
    (parent.type === 'MemberExpression' && parent.object === node) ||
    (parent.type === 'CallExpression' && parent.callee === node) ||
    (parent.type === 'NewExpression' && parent.callee === node) ||
    (parent.type === 'BinaryExpression' &&
      parent.operator === '**' &&
      parent.left === node)
  );
}

parens.set('UnaryExpression', UnaryExpression);
parens.set('SpreadElement', UnaryExpression);
parens.set('BindingObjectPatternRestProperty', UnaryExpression);
parens.set('SpreadProperty', UnaryExpression);

parens.set('FunctionExpression', function FunctionExpression(
  node: AnyNode,
  parent: AnyNode,
  printStack: Array<AnyNode>,
): boolean {
  return isFirstInStatement(printStack, false, true);
});

parens.set('ArrowFunctionExpression', function ArrowFunctionExpression(
  node: ArrowFunctionExpression,
  parent: AnyNode,
): boolean {
  return (
    parent.type === ' ExportLocalDeclaration' ||
    ConditionalExpression(node, parent)
  );
});

function ConditionalExpression(
  node: ArrowFunctionExpression | AssignmentExpression | ConditionalExpression,
  parent: AnyNode,
): boolean {
  if (
    isUnaryLike(parent) ||
    isBinary(parent) ||
    (parent.type === 'ConditionalExpression' && parent.test === node) ||
    parent.type === 'AwaitExpression' ||
    parent.type === 'TaggedTemplateExpression'
  ) {
    return true;
  }

  return UnaryExpression(node, parent);
}

parens.set('ConditionalExpression', ConditionalExpression);

parens.set('AssignmentExpression', function AssignmentExpression(
  node: AssignmentExpression,
  parent: AnyNode,
): boolean {
  if (node.left.type === 'AssignmentObjectPattern') {
    return true;
  } else {
    return ConditionalExpression(node, parent);
  }
});

function UnionTypeAnnotation(node: UnionTypeAnnotation, parent: AnyNode) {
  return (
    parent.type === 'FlowArrayTypeAnnotation' ||
    parent.type === 'FlowNullableTypeAnnotation' ||
    parent.type === 'IntersectionTypeAnnotation' ||
    parent.type === 'UnionTypeAnnotation'
  );
}

parens.set('UnionTypeAnnotation', UnionTypeAnnotation);
parens.set('IntersectionTypeAnnotation', UnionTypeAnnotation);

parens.set('FlowFunctionTypeAnnotation', function FlowFunctionTypeAnnotation(
  node: FlowFunctionTypeAnnotation,
  parent: AnyNode,
  printStack: Array<AnyNode>,
) {
  // Check if we are the return type of an arrow
  for (const printNode of printStack) {
    if (
      printNode.type === 'ArrowFunctionExpression' &&
      printNode.head.returnType === node
    ) {
      return true;
    }
  }

  // ((a: () => A) => (a: A) => A)
  if (
    node.returnType !== undefined &&
    node.returnType.type === 'FlowFunctionTypeAnnotation'
  ) {
    return true;
  }

  return (
    // (() => A) | (() => B)
    parent.type === 'UnionTypeAnnotation' ||
    // (() => A) & (() => B)
    parent.type === 'IntersectionTypeAnnotation' ||
    // (() => A)[]
    parent.type === 'FlowArrayTypeAnnotation'
  );
});

// Walk up the print stack to deterimine if our node can come first
// in statement.
function isFirstInStatement(
  printStack: Array<AnyNode>,
  considerArrow: boolean,
  considerDefaultExports: boolean,
): boolean {
  let i = printStack.length - 1;
  let node = printStack[i];
  i--;
  let parent = printStack[i];
  while (i > 0) {
    if (
      (parent.type === 'ExpressionStatement' && parent.expression === node) ||
      parent.type === 'TaggedTemplateExpression' ||
      (considerDefaultExports &&
        parent.type === 'ExportDefaultDeclaration' &&
        parent.declaration === node) ||
      (considerArrow &&
        parent.type === 'ArrowFunctionExpression' &&
        parent.body === node)
    ) {
      return true;
    }

    if (
      (parent.type === 'CallExpression' && parent.callee === node) ||
      (parent.type === 'SequenceExpression' &&
        parent.expressions[0] === node) ||
      (parent.type === 'MemberExpression' && parent.object === node) ||
      (isConditional(parent) && parent.test === node) ||
      (isBinary(parent) && parent.left === node) ||
      (parent.type === 'AssignmentExpression' && parent.left === node)
    ) {
      node = parent;
      i--;
      parent = printStack[i];
    } else {
      return false;
    }
  }

  return false;
}
