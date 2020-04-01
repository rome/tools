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
  TSDeclareMethod,
  ObjectMethod,
  AnyBindingPattern,
  ClassMethod,
  PatternMeta,
} from '@romejs/js-ast';
import {
  Tokens,
  operator,
  word,
  space,
  terminatorless,
  breakGroup,
} from '../tokens';

export function buildForXStatementGenerator(op: 'of' | 'in'): GeneratorMethod {
  return function(generator: Generator, node: AnyNode): Tokens {
    node = node.type === 'ForInStatement' ? node : forOfStatement.assert(node);

    const tokens: Tokens = [word('for'), space];

    if (op === 'of' && node.type === 'ForOfStatement' && node.await === true) {
      tokens.push(word('await'));
      tokens.push(space);
    }

    return [
      ...tokens,
      operator('('),
      ...generator.print(node.left, node),
      space,
      word(op),
      space,
      ...generator.print(node.right, node),
      operator(')'),
      space,
      ...generator.print(node.body, node),
    ];
  };
}

export function buildYieldAwaitGenerator(keyword: string): GeneratorMethod {
  return function(generator: Generator, node: AnyNode): Tokens {
      node = node.type === 'YieldExpression'
        ? node
        : awaitExpression.assert(node);

      const tokens: Tokens = [word(keyword)];

      if (node.type === 'YieldExpression' && node.delegate === true) {
        tokens.push(operator('*'));
      }

      if (node.argument) {
        return [
          ...tokens,
          space,
          terminatorless(generator.print(node.argument, node)),
        ];
      } else {
        return tokens;
      }
    };
}

export function buildLabelStatementGenerator(prefix: string): GeneratorMethod {
  return function(generator: Generator, node: AnyNode): Tokens {
      node =
        node.type === 'ContinueStatement' || node.type === 'ReturnStatement' ||
        node.type === 'BreakStatement' ? node : throwStatement.assert(node);

    let tokens: Tokens = [word(prefix)];

    if ((node.type === 'ContinueStatement' || node.type === 'BreakStatement') &&
        node.label !== undefined) {
      tokens.push(space);
      tokens = tokens.concat(generator.print(node.label, node));
    }

    if ((node.type === 'ThrowStatement' || node.type === 'ReturnStatement') &&
          node.argument !==
          undefined) {
      tokens.push(space);
      tokens.push(breakGroup([
        [terminatorless(generator.print(node.argument, node))],
      ]));
    }

    tokens.push(operator(';'));

    return tokens;
  };
}

export function printMethod(
  generator: Generator,
  node: TSDeclareMethod | ClassMethod | ObjectMethod,
): Tokens {
  const kind = node.kind;

  const tokens: Tokens = [];

  if (kind === 'method' && node.head.generator === true) {
    tokens.push(operator('*'));
  }

  if (kind === 'get' || kind === 'set') {
    tokens.push(word(kind));
    tokens.push(space);
  }

  if (node.head.async === true) {
    tokens.push(word('async'));
    tokens.push(space);
  }

  if (node.type === 'TSDeclareMethod') {
    return [...tokens, ...generator.print(node.head, node)];
  }

  return [
    ...tokens,
    ...generator.print(node.key, node),
    ...generator.print(node.head, node),
    space,
    ...generator.print(node.body, node),
  ];
}

export function printBindingPatternParams(
  generator: Generator,
  node: AnyNode,
  params: Array<AnyBindingPattern>,
  rest: undefined | AnyBindingPattern,
): Tokens {
  const group = generator.printCommaList(params, node, {
    trailing: rest === undefined,
  });

  if (rest !== undefined) {
    group.groups.push([operator('...'), ...generator.print(rest, node)]);
  }

  return [group];
}

export function printTSBraced(
  generator: Generator,
  node: AnyNode,
  members: Array<AnyNode>,
): Tokens {
  return [
    operator('{'),
    generator.printJoin(members, node, {
      breakOnNewline: true,
      newline: true,
      priority: true,
      broken: {},
      unbroken: {
        separator: [space],
        trim: ';',
      },
    }),
    operator('}'),
  ];
}

export function printPatternMeta(
  generator: Generator,
  node: AnyNode,
  meta: undefined | PatternMeta,
): Tokens {
  if (generator.options.typeAnnotations && meta !== undefined) {
    let tokens: Tokens = [];
    if (meta.optional) {
      tokens.push(operator('?'));
    }

    return [...tokens, ...generator.printTypeColon(meta.typeAnnotation, node)];
  } else {
    return [];
  }
}
