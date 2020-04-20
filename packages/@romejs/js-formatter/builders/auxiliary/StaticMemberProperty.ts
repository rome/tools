/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {staticMemberProperty, AnyNode} from '@romejs/js-ast';
import {operator, concat} from '@romejs/js-formatter/tokens';

export default function StaticMemberProperty(builder: Builder, node: AnyNode) {
  node = staticMemberProperty.assert(node);

  return [operator('.'), concat(builder.tokenize(node.value, node))];
}
