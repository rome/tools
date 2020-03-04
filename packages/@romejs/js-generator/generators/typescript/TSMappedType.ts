/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMappedType, tsMappedType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {tokenIfPlusMinus} from '../utils';

export default function TSMappedType(generator: Generator, node: AnyNode) {
  node = tsMappedType.assert(node);

  generator.token('{');
  generator.space();

  if (node.readonly) {
    tokenIfPlusMinus(generator, node.readonly);
    generator.word('readonly');
    generator.space();
  }

  const {typeParameter} = node;
  generator.token('[');
  generator.word(typeParameter.name);
  generator.space();
  generator.word('in');
  generator.space();
  generator.print(typeParameter.constraint, typeParameter);
  generator.token(']');

  if (node.optional) {
    tokenIfPlusMinus(generator, node.optional);
    generator.token('?');
  }

  generator.token(':');
  generator.space();
  generator.print(node.typeAnnotation, node);
  generator.space();
  generator.token('}');
}
