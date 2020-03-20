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
    node = node.type === 'YieldExpression' ? node : awaitExpression.assert(node);

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
      node.type === 'ContinueStatement' || node.type === 'ReturnStatement' ||
      node.type === 'BreakStatement' ? node : throwStatement.assert(node);

    generator.word(prefix);

    if ((node.type === 'ContinueStatement' || node.type === 'BreakStatement') &&
      node.label !== undefined) {
      generator.space();
      generator.print(node.label, node);
    }

    if ((node.type === 'ThrowStatement' || node.type === 'ReturnStatement') &&
      node.argument !== undefined) {
      generator.space();

      generator.multiline(node, (multiline, node) => {
        const terminatorState = generator.startTerminatorless();
        if (multiline) {
          generator.forceNewline();
        }
        generator.print(node.argument, node);
        generator.endTerminatorless(terminatorState);
      });
    }

    generator.semicolon();
  };
}

export function printMethod(
  generator: Generator,
  node: TSDeclareMethod | ClassMethod | ObjectMethod,
) {
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

  if (node.type === 'TSDeclareMethod') {
    generator.print(node.head, node);
    return;
  }

  generator.print(node.key, node);
  generator.print(node.head, node);
  generator.space();
  generator.print(node.body, node);
}

export function tokenIfPlusMinus(generator: Generator, token: string | true) {
  if (token !== true) {
    generator.token(token);
  }
}

export function printBindingPatternParams(
  generator: Generator,
  node: AnyNode,
  params: Array<AnyBindingPattern>,
  rest: undefined | AnyBindingPattern,
  multiline: boolean = false,
) {
  generator.printCommaList(params, node, {
    trailing: true,
    multiline,
  });

  if (rest !== undefined) {
    if (params.length > 0) {
      if (!multiline) {
        generator.token(',');
      }
      generator.spaceOrNewline(multiline);
    }

    generator.token('...');
    generator.print(rest, node);
  }
}

export function printTSBraced(
  generator: Generator,
  node: AnyNode,
  members: Array<AnyNode>,
) {
  generator.token('{');

  if (members.length > 0) {
    const multiline = members.length > 1;

    generator.indent();

    if (multiline) {
      generator.newline();
    }

    for (const member of members) {
      generator.print(member, node);

      if (multiline) {
        generator.newline();
      } else {
        generator.buf.removeTrailing(';');
      }
    }

    generator.dedent();
    generator.rightBrace();
  } else {
    generator.token('}');
  }
}

export function printPatternMeta(
  generator: Generator,
  node: AnyNode,
  meta: undefined | PatternMeta,
) {
  if (generator.options.typeAnnotations && meta !== undefined) {
    if (meta.optional) {
      generator.token('?');
    }
    generator.printTypeColon(meta.typeAnnotation, node);
  }
}
