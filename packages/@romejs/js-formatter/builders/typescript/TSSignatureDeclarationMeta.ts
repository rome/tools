/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {
  TSSignatureDeclarationMeta,
  tsSignatureDeclarationMeta,
  AnyNode,
} from '@romejs/js-ast';
import {printBindingPatternParams} from '../utils';

export default function TSSignatureDeclarationMeta(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsSignatureDeclarationMeta.assert(node);

  return [
    ...builder.tokenize(node.typeParameters, node),
    operator('('),
    ...printBindingPatternParams(builder, node, node.parameters, node.rest),
    operator(')'),
  ];
}
