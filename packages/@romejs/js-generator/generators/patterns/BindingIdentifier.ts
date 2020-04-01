/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {BindingIdentifier, bindingIdentifier, AnyNode} from '@romejs/js-ast';
import Identifier from '../auxiliary/Identifier';
import {printPatternMeta} from '../utils';

export default function BindingIdentifier(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = bindingIdentifier.assert(node);

  if (node.name[0] === '*') {
    // Internal name
    return [];
  }

  return [
    ...Identifier(generator, node),
    ...printPatternMeta(generator, node, node.meta),
  ];
}
