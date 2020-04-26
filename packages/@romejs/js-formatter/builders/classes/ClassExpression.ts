/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {ClassExpression} from '@romejs/js-ast';
import ClassDeclaration from './ClassDeclaration';

export default function ClassExpression(
  builder: Builder,
  node: ClassExpression,
): Token {
  return ClassDeclaration(builder, node);
}
