/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ImportDeclaration, importDeclaration, AnyNode} from '@romejs/js-ast';

export default function ImportDeclaration(generator: Generator, node: AnyNode) {
  node = importDeclaration.assert(node);

  generator.word('import');
  generator.space();

  if (node.importKind === 'type' || node.importKind === 'typeof') {
    generator.word(node.importKind);
    generator.space();
  }

  let {specifiers} = node;
  if (specifiers !== undefined && specifiers.length > 0) {
    specifiers = [...specifiers];

    // Print "special" specifiers first
    while (specifiers.length > 0) {
      const first = specifiers[0];
      if (
        first.type === 'ImportDefaultSpecifier' ||
        first.type === 'ImportNamespaceSpecifier'
      ) {
        generator.print(specifiers.shift(), node);
        if (specifiers.length) {
          generator.token(',');
          generator.space();
        }
      } else {
        break;
      }
    }

    if (specifiers.length > 0) {
      generator.token('{');
      generator.printCommaList(specifiers, node);
      generator.token('}');
    }

    generator.space();
    generator.word('from');
    generator.space();
  }

  generator.print(node.source, node);
  generator.semicolon();
}
