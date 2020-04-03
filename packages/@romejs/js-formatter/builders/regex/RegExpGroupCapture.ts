/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, verbatim} from '../../tokens';
import {AnyNode, RegExpGroupCapture, regExpGroupCapture} from '@romejs/js-ast';

export default function RegExpGroupCapture(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = regExpGroupCapture.assert(node);

  const tokens: Tokens = [verbatim('(')];

  if (node.name !== undefined) {
    tokens.push(verbatim('?<'));
    tokens.push(verbatim(node.name));
    tokens.push(verbatim('>'));
  }

  return [...tokens, ...builder.tokenize(node.expression, node), verbatim(')')];
}
