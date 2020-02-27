/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator, {GeneratorMethod} from '../Generator';
import {
  awaitExpression,
  forOfStatement,
  AnyNode,
  throwStatement,
  objectMethod,
} from '@romejs/js-ast';

export function buildForXStatementGenerator(op: 'of' | 'in'): GeneratorMethod {
  return function(generator: Generator, node: AnyNode) {
    node = node.type === 'ForInStatement' ? node : forOfStatement.assert(node);

    generator.word('for');
    generator.space();
    if (op === 'of' && node.type === 'ForOfStatement' && node.await === true) {
      generator.word('await');
      generator.space();
    }
    generator.token('(');
    generator.print(node.left, node);
    generator.space();
    generator.word(op);
    generator.space();
    generator.print(node.right, node);
    generator.token(')');
    generator.printBlock(node);
  };
}

export function buildYieldAwaitGenerator(keyword: string): GeneratorMethod {
  return function(generator: Generator, node: AnyNode) {
    node =
      node.type === 'YieldExpression' ? node : awaitExpression.assert(node);

    generator.word(keyword);

    if (node.type === 'YieldExpression' && node.delegate === true) {
      generator.token('*');
    }

    if (node.argument) {
      generator.space();
      const terminatorState = generator.startTerminatorless();
      generator.print(node.argument, node);
      generator.endTerminatorless(terminatorState);
    }
  };
}

export function buildLabelStatementGenerator(prefix: string): GeneratorMethod {
  return function(generator: Generator, node: AnyNode) {
    node =
      node.type === 'ContinueStatement' ||
      node.type === 'ReturnStatement' ||
      node.type === 'BreakStatement'
        ? node
        : throwStatement.assert(node);

    generator.word(prefix);

    let arg: undefined | AnyNode;
    if (
      (node.type === 'ContinueStatement' || node.type === 'BreakStatement') &&
      node.label !== undefined
    ) {
      arg = node.label;
    }
    if (
      (node.type === 'ThrowStatement' || node.type === 'ReturnStatement') &&
      node.argument !== undefined
    ) {
      arg = node.argument;
    }

    if (arg !== undefined) {
      generator.space();

      const terminatorState = generator.startTerminatorless();
      generator.print(arg, node);
      generator.endTerminatorless(terminatorState);
    }

    generator.semicolon();
  };
}

export function printMethod(generator: Generator, node: AnyNode) {
  node = node.type === 'ClassMethod' ? node : objectMethod.assert(node);

  const kind = node.kind;

  if (kind === 'method' && node.head.generator === true) {
    generator.token('*');
  }

  if (kind === 'get' || kind === 'set') {
    generator.word(kind);
    generator.space();
  }

  if (node.head.async === true) {
    generator.word('async');
    generator.space();
  }

  generator.print(node.key, node);
  generator.print(node.head, node);
  generator.space();
  generator.print(node.body, node);
}
