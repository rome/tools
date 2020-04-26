/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {FlowVariance} from '@romejs/js-ast';

export default function FlowVariance(
  builder: Builder,
  node: FlowVariance,
): Token {
  return node.kind === 'plus' ? '+' : '-';
}
