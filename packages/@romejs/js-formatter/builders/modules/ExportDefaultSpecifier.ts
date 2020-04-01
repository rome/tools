/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  ExportDefaultSpecifier,
  exportDefaultSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ExportDefaultSpecifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = exportDefaultSpecifier.assert(node);
  return builder.print(node.exported, node);
}
