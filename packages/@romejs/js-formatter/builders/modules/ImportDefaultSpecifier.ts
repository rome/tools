/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  ImportDefaultSpecifier,
  importDefaultSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ImportDefaultSpecifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = importDefaultSpecifier.assert(node);
  return builder.tokenize(node.local.name, node);
}
