/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSConstructorType, tsConstructorType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens} from '../../tokens';

export default function TSConstructorType(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsConstructorType.assert(node);

  throw new Error('unimplemented');
}
