/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {operator, concat} from '../../tokens';
import {AnyNode} from '@romejs/js-ast';
import StringLiteral from '../literals/StringLiteral';

export default function Directive(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
) {
  return [concat(StringLiteral(builder, node, parent)), operator(';')];
}
