/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {jsxReferenceIdentifier, AnyNode} from '@romejs/js-ast';

export default function JSXReferenceIdentifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = jsxReferenceIdentifier.assert(node);
  throw new Error('unimplemented');
}
