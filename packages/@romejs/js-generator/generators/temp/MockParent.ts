/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {MockParent, mockParent, AnyNode} from '@romejs/js-ast';

export default function MockParent(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = mockParent.assert(node);
  mockParent.assert(node);
  throw new Error('unimplemented');
}
