/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSInterfaceBody, tsInterfaceBody, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {printTSBraced} from '../utils';

export default function TSInterfaceBody(generator: Generator, node: AnyNode) {
  node = tsInterfaceBody.assert(node);
  printTSBraced(generator, node.body, node);
}
