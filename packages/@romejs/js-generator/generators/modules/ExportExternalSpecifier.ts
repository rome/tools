/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {
  AnyNode,
  ExportExternalSpecifier,
  exportExternalSpecifier,
} from '@romejs/js-ast';
import ExportLocalSpecifier from './ExportLocalSpecifier';

export default function ExportExternalSpecifier(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = exportExternalSpecifier.assert(node);
  return ExportLocalSpecifier(generator, node);
}
