/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {BindingIdentifier} from '@romejs/js-ast';
import Identifier from '../auxiliary/Identifier';
import {printPatternMeta} from '../utils';

export default function BindingIdentifier(
  builder: Builder,
  node: BindingIdentifier,
): Token {
  if (node.name[0] === '*') {
    // Internal name
    return '';
  }

  return concat([
    Identifier(builder, node),
    printPatternMeta(builder, node, node.meta),
  ]);
}
