/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSModuleBlock} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token} from '../../tokens';
import {printTSBraced} from '../utils';

export default function TSModuleBlock(
  builder: Builder,
  node: TSModuleBlock,
): Token {
  return printTSBraced(builder, node, node.body);
}
