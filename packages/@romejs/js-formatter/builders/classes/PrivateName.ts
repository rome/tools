/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {AnyNode, privateName} from '@romejs/js-ast';
import {operator} from '@romejs/js-formatter/tokens';

export default function PrivateName(builder: Builder, node: AnyNode) {
  node = privateName.assert(node);

  return [operator('#'), ...builder.print(node.id, node)];
}
