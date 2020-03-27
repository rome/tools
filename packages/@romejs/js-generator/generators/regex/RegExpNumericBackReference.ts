/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import Generator from '../../Generator';
import {
  AnyNode,
  RegExpNumericBackReference,
  regExpNumericBackReference,
} from '@romejs/js-ast';

export default function RegExpNumericBackReference(
  generator: Generator,
  node: AnyNode,
) {
  node = regExpNumericBackReference.assert(node);
  return generator.append(`\\${node.value}`);
}
