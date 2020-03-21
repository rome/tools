/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  TSSignatureDeclarationMeta,
  tsSignatureDeclarationMeta,
  AnyNode,
} from '@romejs/js-ast';
import {printBindingPatternParams} from '../utils';

export default function TSSignatureDeclarationMeta(
  generator: Generator,
  node: AnyNode,
) {
  node = tsSignatureDeclarationMeta.assert(node);

  generator.print(node.typeParameters, node);
  generator.token('(');
  printBindingPatternParams(generator, node, node.parameters, node.rest);
  generator.token(')');
  generator.space();
}
