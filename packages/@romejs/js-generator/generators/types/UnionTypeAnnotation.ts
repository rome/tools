/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {UnionTypeAnnotation, unionTypeAnnotation, AnyNode} from '@romejs/js-ast';

export default function UnionTypeAnnotation(generator: Generator, node: AnyNode) {
  node = unionTypeAnnotation.assert(node);

  generator.multiline(node, (multiline, node) => {
    if (multiline) {
      generator.indent();
      orNewlineSeparator(generator, false);
    }

    generator.printJoin(node.types, node, {
      after: multiline ? orNewlineSeparator : orSpaceSeparator,
    });

    if (multiline) {
      generator.dedent();
    }
  }, {conditions: ['more-than-one-line'], indent: true});
}

function orNewlineSeparator(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  generator.newline();
  generator.token('|');
  generator.space();
}

function orSpaceSeparator(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  generator.space();
  generator.token('|');
  generator.space();
}
