/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {BlockStatement, blockStatement, AnyNode} from '@romejs/js-ast';

export default function BlockStatement(generator: Generator, node: AnyNode) {
  node = blockStatement.assert(node);

  generator.token('{');
  generator.printInnerComments(node);

  const hasDirectives: boolean = Boolean(
    node.directives && node.directives.length > 0,
  );

  if (node.body.length > 0 || hasDirectives) {
    generator.newline();

    generator.printStatementList(node.directives, node, {indent: true});
    if (hasDirectives) {
      generator.newline();
    }

    generator.printStatementList(node.body, node, {indent: true});
    generator.removeTrailingNewline();

    generator.source('end', node.loc);

    if (!generator.endsWith('\n')) {
      generator.newline();
    }

    generator.rightBrace();
  } else {
    generator.source('end', node.loc);
    generator.token('}');
  }
}
