/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {BindingIdentifier, bindingIdentifier, AnyNode} from '@romejs/js-ast';
import Identifier from '../auxiliary/Identifier';

export default function BindingIdentifier(generator: Generator, node: AnyNode) {
  node = bindingIdentifier.assert(node);

  if (node.name[0] === '*') {
    // Internal name
    return;
  }

  Identifier(generator, node);
}
