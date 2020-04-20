/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {MockParent, mockParent, AnyNode} from '@romejs/js-ast';

export default function MockParent(builder: Builder, node: AnyNode): Tokens {
  node = mockParent.assert(node);
  mockParent.assert(node);
  throw new Error('unimplemented');
}
