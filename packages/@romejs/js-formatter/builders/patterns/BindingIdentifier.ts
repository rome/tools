/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat} from '../../tokens';
import {bindingIdentifier, AnyNode} from '@romejs/js-ast';
import Identifier from '../auxiliary/Identifier';
import {printPatternMeta} from '../utils';

export default function BindingIdentifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = bindingIdentifier.assert(node);

  if (node.name[0] === '*') {
    // Internal name
    return [];
  }

  return [
    concat(Identifier(builder, node)),
    concat(printPatternMeta(builder, node, node.meta)),
  ];
}
