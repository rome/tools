/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import Generator from '../../Generator';
import {
  AnyNode,
  RegExpNamedBackReference,
  regExpNamedBackReference,
} from '@romejs/js-ast';

export default function RegExpNamedBackReference(
  generator: Generator,
  node: AnyNode,
) {
  node = regExpNamedBackReference.assert(node);
  generator.append('\\k');
  generator.append('<');
  generator.append(node.name);
  generator.append('>');
}
