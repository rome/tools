/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ExportNamedDeclaration,
  exportNamedDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {isStatement} from '@romejs/js-ast-utils';
export default function ExportNamedDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = exportNamedDeclaration.assert(node);

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
      : exportNamedDeclaration.assert(node);

  if (node.declaration) {
    const declar = node.declaration;
    generator.print(declar, node);
    if (!isStatement(declar)) {
      generator.semicolon();
    }
  } else {
    if (node.type !== 'ExportNamedDeclaration') {
      throw new Error('Expected ExportNamedDeclaration');
    }

    if (node.exportKind === 'type') {
      generator.word('type');
      generator.space();
    }

    if (node.specifiers === undefined) {
      throw new Error('Expected specifiers since there was no declaration');
    }

    const specifiers = node.specifiers.slice(0);

    // print "special" specifiers first
    let hasSpecial = false;
    while (true) {
      const first = specifiers[0];
      if (
        first !== undefined &&
        (first.type === 'ExportDefaultSpecifier' ||
          first.type === 'ExportNamespaceSpecifier')
      ) {
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

    if (specifiers.length || (!specifiers.length && !hasSpecial)) {
      generator.token('{');
      if (specifiers.length) {
        generator.printCommaList(specifiers, node);
      }
      generator.token('}');
    }

    if (node.source) {
      generator.space();
      generator.word('from');
      generator.space();
      generator.print(node.source, node);
    }

    generator.semicolon();
  }
}
