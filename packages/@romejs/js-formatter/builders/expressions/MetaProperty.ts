/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator} from '../../tokens';
import {AnyNode, metaProperty} from '@romejs/js-ast';

export default function MetaProperty(builder: Builder, node: AnyNode): Tokens {
  node = metaProperty.assert(node);

  return [
    concat(builder.tokenize(node.meta, node)),
    operator('.'),
    concat(builder.tokenize(node.property, node)),
  ];
}
