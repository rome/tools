/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {FlowClassImplements} from '@romejs/js-ast';
import FlowInterfaceExtends from './FlowInterfaceExtends';

export default function FlowClassImplements(
  builder: Builder,
  node: FlowClassImplements,
): Token {
  return FlowInterfaceExtends(builder, node);
}
