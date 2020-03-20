/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AnyNode,
  ExportExternalDeclaration,
  exportExternalDeclaration,
} from '@romejs/js-ast';

export default function ExportExternalDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = exportExternalDeclaration.assert(node);

  generator.word('export');
  generator.space();

  if (node.exportKind === 'type') {
    generator.word('type');
    generator.space();
  }

  if (node.specifiers === undefined) {
    throw new Error('Expected specifiers since there was no declaration');
  }

  const specifiers = node.specifiers.slice(0);

  generator.multiline(node, (multiline, node) => {
    // print "special" specifiers first
    let hasSpecial = false;
    while (true) {
      const first = specifiers[0];
      if (first !== undefined && (first.type === 'ExportDefaultSpecifier' ||
      first.type === 'ExportNamespaceSpecifier')) {
        hasSpecial = true;
        generator.print(specifiers.shift(), node);
        if (specifiers.length) {
          generator.token(',');
          generator.space();
        }
      } else {
        break;
      }
    }

    if (specifiers.length || !specifiers.length && !hasSpecial) {
      generator.token('{');
      if (specifiers.length) {
        generator.printCommaList(specifiers, node, {
          multiline,
          trailing: true,
        });
      }
      generator.token('}');
    }

    generator.space();
    generator.word('from');
    generator.space();
    generator.print(node.source, node);

    generator.semicolon();
  });
}
