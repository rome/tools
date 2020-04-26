/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowTypeofTypeAnnotation} from '@romejs/js-ast';

export default function FlowTypeofTypeAnnotation(
  builder: Builder,
  node: FlowTypeofTypeAnnotation,
): Token {
  return concat(['typeof', space, builder.tokenize(node.argument, node)]);
}
