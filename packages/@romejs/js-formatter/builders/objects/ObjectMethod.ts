/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {printMethod} from '../utils';
import {ObjectMethod, objectMethod, AnyNode} from '@romejs/js-ast';

export default function ObjectMethod(builder: Builder, node: AnyNode): Tokens {
  node = objectMethod.assert(node);
  return printMethod(builder, node);
}
