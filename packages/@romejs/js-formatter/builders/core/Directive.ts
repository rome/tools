/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {AnyNode, Directive} from '@romejs/js-ast';
import StringLiteral from '../literals/StringLiteral';

export default function Directive(
  builder: Builder,
  node: Directive,
  parent: AnyNode,
): Token {
  return concat([StringLiteral(builder, node, parent), ';']);
}
