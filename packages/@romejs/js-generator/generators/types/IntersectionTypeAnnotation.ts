/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  IntersectionTypeAnnotation,
  intersectionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function IntersectionTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = intersectionTypeAnnotation.assert(node);

  generator.multiline(
    node,
    (multiline, node) => {
      if (multiline) {
        andNewlineSeparator(generator, false);
      }

      generator.printJoin(node.types, node, {
        after: multiline ? andNewlineSeparator : andSpaceSeparator,
      });
    },
    {conditions: ['more-than-one-line'], indent: true},
  );
}

function andNewlineSeparator(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  generator.newline();
  generator.token('&');
  generator.space();
}

function andSpaceSeparator(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  generator.space();
  generator.token('&');
  generator.space();
}
