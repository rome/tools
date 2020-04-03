/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {PatternMeta, patternMeta, AnyNode} from '@romejs/js-ast';

export default function PatternMeta(builder: Builder, node: AnyNode): Tokens {
  node = patternMeta.assert(node);
  patternMeta.assert(node);
  throw new Error('unimplemented');
}
