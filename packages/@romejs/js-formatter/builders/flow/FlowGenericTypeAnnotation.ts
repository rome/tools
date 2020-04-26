/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {FlowGenericTypeAnnotation} from '@romejs/js-ast';
import FlowInterfaceExtends from './FlowInterfaceExtends';

export default function FlowGenericTypeAnnotation(
  builder: Builder,
  node: FlowGenericTypeAnnotation,
): Token {
  return FlowInterfaceExtends(builder, node);
}
