/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder, {BuilderMethod} from '../Builder';
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

export function buildForXStatementBuilder(op: 'of' | 'in'): BuilderMethod {
  return function(builder: Builder, node: AnyNode): Tokens {
    node = node.type === 'ForInStatement' ? node : forOfStatement.assert(node);

    const tokens: Tokens = [word('for'), space];

    if (op === 'of' && node.type === 'ForOfStatement' && node.await === true) {
      tokens.push(word('await'));
      tokens.push(space);
    }

    return [
      ...tokens,
      operator('('),
      ...builder.tokenize(node.left, node),
      space,
      word(op),
      space,
      ...builder.tokenize(node.right, node),
      operator(')'),
      space,
      ...builder.tokenize(node.body, node),
    ];
  };
}

export function buildYieldAwaitBuilder(keyword: string): BuilderMethod {
  return function(builder: Builder, node: AnyNode): Tokens {
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
          terminatorless(builder.tokenize(node.argument, node)),
        ];
      } else {
        return tokens;
      }
    };
}

export function buildLabelStatementBuilder(prefix: string): BuilderMethod {
  return function(builder: Builder, node: AnyNode): Tokens {
      node =
        node.type === 'ContinueStatement' || node.type === 'ReturnStatement' ||
        node.type === 'BreakStatement' ? node : throwStatement.assert(node);

    let tokens: Tokens = [word(prefix)];

    if ((node.type === 'ContinueStatement' || node.type === 'BreakStatement') &&
        node.label !== undefined) {
      tokens.push(space);
      tokens = tokens.concat(builder.tokenize(node.label, node));
    }

    if ((node.type === 'ThrowStatement' || node.type === 'ReturnStatement') &&
          node.argument !==
          undefined) {
      tokens.push(space);
      tokens.push(breakGroup([
        [terminatorless(builder.tokenize(node.argument, node))],
      ]));
    }

    tokens.push(operator(';'));

    return tokens;
  };
}

export function printMethod(
  builder: Builder,
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
    return [...tokens, ...builder.tokenize(node.head, node)];
  }

  return [
    ...tokens,
    ...builder.tokenize(node.key, node),
    ...builder.tokenize(node.head, node),
    space,
    ...builder.tokenize(node.body, node),
  ];
}

export function printBindingPatternParams(
  builder: Builder,
  node: AnyNode,
  params: Array<AnyBindingPattern>,
  rest: undefined | AnyBindingPattern,
): Tokens {
  const group = builder.tokenizeCommaList(params, node, {
    trailing: rest === undefined,
  });

  if (rest !== undefined) {
    group.groups.push([operator('...'), ...builder.tokenize(rest, node)]);
  }

  return [group];
}

export function printTSBraced(
  builder: Builder,
  node: AnyNode,
  members: Array<AnyNode>,
): Tokens {
  return [
    operator('{'),
    builder.tokenizeJoin(members, node, {
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
  builder: Builder,
  node: AnyNode,
  meta: undefined | PatternMeta,
): Tokens {
  if (builder.options.typeAnnotations && meta !== undefined) {
    let tokens: Tokens = [];
    if (meta.optional) {
      tokens.push(operator('?'));
    }

    return [...tokens, ...builder.tokenizeTypeColon(meta.typeAnnotation, node)];
  } else {
    return [];
  }
}
