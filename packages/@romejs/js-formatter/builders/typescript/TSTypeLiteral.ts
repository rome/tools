/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeLiteral} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token} from '../../tokens';
import {printTSBraced} from '../utils';

export default function TSTypeLiteral(
  builder: Builder,
  node: TSTypeLiteral,
): Token {
  return printTSBraced(builder, node, node.members);
}
