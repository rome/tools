/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {FlowObjectTypeAnnotation} from '@romejs/js-ast';
import {printCommaList} from '../utils';

export default function FlowObjectTypeAnnotation(
  builder: Builder,
  node: FlowObjectTypeAnnotation,
): Token {
  return concat([
    node.exact ? '{|' : '{',
    printCommaList(builder, node.properties, node),
    node.exact ? '|}' : '}',
  ]);
}
