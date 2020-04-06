/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AnyNode,
  ExportExternalSpecifier,
  exportExternalSpecifier,
} from '@romejs/js-ast';
import ExportLocalSpecifier from './ExportLocalSpecifier';

export default function ExportExternalSpecifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = exportExternalSpecifier.assert(node);
  return ExportLocalSpecifier(builder, node);
}
