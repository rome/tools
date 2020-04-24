/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSConstructorType, tsConstructorType} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens} from '../../tokens';

export default function TSConstructorType(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsConstructorType.assert(node);

  throw new Error('unimplemented');
}
