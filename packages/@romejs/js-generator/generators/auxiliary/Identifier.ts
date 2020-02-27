/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, Identifier, identifier} from '@romejs/js-ast';

export default function Identifier(generator: Generator, node: AnyNode) {
  node =
    node.type === 'ReferenceIdentifier' ||
    node.type === 'BindingIdentifier' ||
    node.type === 'AssignmentIdentifier'
      ? node
      : identifier.assert(node);

  generator.word(node.name);
}
