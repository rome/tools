/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {PatternMeta, patternMeta, AnyNode} from '@romejs/js-ast';

export default function PatternMeta(generator: Generator, node: AnyNode) {
  node = patternMeta.assert(node);
  patternMeta.assert(node);
  throw new Error('unimplemented');
}
