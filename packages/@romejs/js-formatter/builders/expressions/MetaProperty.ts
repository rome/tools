/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {MetaProperty, metaProperty, AnyNode} from '@romejs/js-ast';

export default function MetaProperty(builder: Builder, node: AnyNode): Tokens {
  node = metaProperty.assert(node);

  return [
    ...builder.print(node.meta, node),
    operator('.'),
    ...builder.print(node.property, node),
  ];
}
