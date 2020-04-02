/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, privateName} from '@romejs/js-ast';

export default function PrivateName(generator: Generator, node: AnyNode) {
  node = privateName.assert(node);

  generator.token('#');
  generator.print(node.id, node);
}
