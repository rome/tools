/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AnyNode,
  RegExpGroupNonCapture,
  regExpGroupNonCapture,
} from '@romejs/js-ast';

export default function RegExpGroupNonCapture(
  generator: Generator,
  node: AnyNode,
) {
  node = regExpGroupNonCapture.assert(node);

  generator.append('(?');

  switch (node.kind) {
    case 'positive-lookahead':
      generator.append('=');
      break;

    case 'negative-lookahead':
      generator.append('!');
      break;

    case 'positive-lookbehind':
      generator.append('<!');
      break;

    case 'negative-lookbehind':
      generator.append('<=');
      break;

    default:
      generator.append(':');
  }

  generator.print(node.expression, node);
  generator.append(')');
}
