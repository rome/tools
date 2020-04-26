/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowDeclareInterface} from '@romejs/js-ast';
import FlowInterfaceDeclaration from './FlowInterfaceDeclaration';

export default function FlowDeclareInterface(
  builder: Builder,
  node: FlowDeclareInterface,
): Token {
  return concat(['declare', space, FlowInterfaceDeclaration(builder, node)]);
}
