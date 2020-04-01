/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {classExpression, AnyNode} from '@romejs/js-ast';
import ClassDeclaration from './ClassDeclaration';

export default function ClassExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = classExpression.assert(node);
  return ClassDeclaration(generator, node);
}
