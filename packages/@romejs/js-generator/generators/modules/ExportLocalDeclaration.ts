/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ExportLocalDeclaration,
  exportLocalDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {isStatement} from '@romejs/js-ast-utils';
export default function ExportLocalDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = exportLocalDeclaration.assert(node);

  if (node.exportKind === 'type' && !generator.options.typeAnnotations) {
    return;
  }
  generator.word('export');
  generator.space();
  _ExportDeclaration(generator, node);
}

export function _ExportDeclaration(generator: Generator, node: AnyNode) {
  node =
    node.type === 'ExportDefaultDeclaration'
      ? node
      : exportLocalDeclaration.assert(node);

  if (node.declaration) {
    const declar = node.declaration;
    generator.print(declar, node);
    if (!isStatement(declar)) {
      generator.semicolon();
    }
  } else {
    if (node.type !== 'ExportLocalDeclaration') {
      throw new Error('Expected  ExportLocalDeclaration');
    }

    if (node.exportKind === 'type') {
      generator.word('type');
      generator.space();
    }

    if (node.specifiers === undefined) {
      throw new Error('Expected specifiers since there was no declaration');
    }

    generator.token('{');
    if (node.specifiers.length > 0) {
      generator.printCommaList(node.specifiers, node);
    }
    generator.token('}');

    generator.semicolon();
  }
}
