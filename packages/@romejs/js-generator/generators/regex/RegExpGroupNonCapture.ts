/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, verbatim} from '../../tokens';
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

  const tokens: Tokens = [verbatim('(?')];

  switch (node.kind) {
    case 'positive-lookahead':
      tokens.push(verbatim('='));
      break;

    case 'negative-lookahead':
      tokens.push(verbatim('!'));
      break;

    case 'positive-lookbehind':
      tokens.push(verbatim('<!'));
      break;

    case 'negative-lookbehind':
      tokens.push(verbatim('<='));
      break;

    default:
      tokens.push(verbatim(':'));
  }

  return [...tokens, ...generator.print(node.expression, node), verbatim(')')];
}
