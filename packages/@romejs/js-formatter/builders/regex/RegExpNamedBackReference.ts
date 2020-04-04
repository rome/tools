/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import Builder from '../../Builder';
import {Tokens, verbatim} from '../../tokens';
import {
  AnyNode,
  RegExpNamedBackReference,
  regExpNamedBackReference,
} from '@romejs/js-ast';

export default function RegExpNamedBackReference(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = regExpNamedBackReference.assert(node);
  return [
    verbatim('\\k'),
    verbatim('<'),
    verbatim(node.name),
    verbatim('>'),
  ];
}
