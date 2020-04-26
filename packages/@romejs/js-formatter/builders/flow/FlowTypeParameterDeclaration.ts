/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {FlowTypeParameterDeclaration} from '@romejs/js-ast';
import FlowTypeParameterInstantiation from './FlowTypeParameterInstantiation';

export default function FlowTypeParameterDeclaration(
  builder: Builder,
  node: FlowTypeParameterDeclaration,
): Token {
  return FlowTypeParameterInstantiation(builder, node);
}
