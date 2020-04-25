/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {FlowTupleTypeAnnotation} from '@romejs/js-ast';
import {printCommaList} from '../utils';

export default function FlowTupleTypeAnnotation(
  builder: Builder,
  node: FlowTupleTypeAnnotation,
): Token {
  return concat(['[', printCommaList(builder, node.types, node), ']']);
}
