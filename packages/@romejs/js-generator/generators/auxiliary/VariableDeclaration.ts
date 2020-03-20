/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, VariableDeclaration, variableDeclaration} from '@romejs/js-ast';
import {isFor} from '@romejs/js-ast-utils';

export default function VariableDeclaration(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = variableDeclaration.assert(node);

  generator.word(node.kind);
  generator.space();

  let hasInits = false;

  // don't add whitespace to loop heads
  if (!isFor(parent)) {
    for (const declar of node.declarations) {
      if (declar.init) {
        // has an init so let's split it up over multiple lines
        hasInits = true;
      }
    }
  }

  //

  // use a pretty separator when we aren't in compact mode, have initializers and don't have retainLines on

  // this will format declarations like:
  //

  //   let foo = "bar", bar = "foo";

  //

  // into

  //

  //   let foo = "bar",

  //       bar = "foo";

  //
  let separator = variableDeclarationNormal;
  if (hasInits) {
    separator = node.kind === 'const'
      ? constDeclarationIndent : variableDeclarationIndent;
  }

  generator.printJoin(node.declarations, node, {
    after: separator,
  });
}

function variableDeclarationNormal(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  generator.token(',');
  generator.space();
}

function variableDeclarationIndent(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  // "let " or "var " indentation.
  generator.token(',');
  generator.forceNewline();
  if (generator.buf.endsWith('\n')) {
    for (let i = 0; i < 4; i++) {
      generator.space();
    }
  }
}

function constDeclarationIndent(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  // "const " indentation.
  generator.token(',');
  generator.forceNewline();
  if (generator.buf.endsWith('\n')) {
    for (let i = 0; i < 6; i++) {
      generator.space();
    }
  }
}
