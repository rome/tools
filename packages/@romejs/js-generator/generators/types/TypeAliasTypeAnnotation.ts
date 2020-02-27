/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  TypeAliasTypeAnnotation,
  typeAliasTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function TypeAliasTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = typeAliasTypeAnnotation.assert(node);

  generator.word('type');
  generator.space();
  generator.print(node.id, node);
  generator.print(node.typeParameters, node);
  generator.space();
  generator.token('=');
  generator.space();
  generator.print(node.right, node);
  generator.semicolon();
}
