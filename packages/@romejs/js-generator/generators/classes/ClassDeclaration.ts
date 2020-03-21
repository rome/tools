/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ClassDeclaration, classDeclaration, AnyNode} from '@romejs/js-ast';

export default function ClassDeclaration(generator: Generator, node: AnyNode) {
  node = node.type === 'ClassExpression' ? node : classDeclaration.assert(node);

  generator.word('class');

  if (node.id) {
    generator.space();
    generator.print(node.id, node);
  }

  generator.print(node.meta, node);

  generator.space();

  generator.token('{');

  generator.printInnerComments(node);
  generator.printInnerComments(node.meta);

  if (node.meta.body.length === 0) {
    generator.token('}');
  } else {
    generator.forceNewline();

    generator.indent();
    generator.printStatementList(node.meta.body, node.meta);
    generator.dedent();

    generator.buf.removeTrailingNewlines();
    generator.forceNewline();

    generator.rightBrace();
  }
}
