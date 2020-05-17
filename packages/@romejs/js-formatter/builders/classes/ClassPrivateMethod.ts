/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Token, concat, space} from '../../tokens';
import { Builder } from '@romejs/js-formatter';
import { ClassPrivateMethod } from '@romejs/js-ast';
import { printMethod } from '../utils';

export default function ClassPrivateMethod(
  builder: Builder,
  node: ClassPrivateMethod
): Token {
  const printed = printMethod(builder, node);
  if (node.meta.static === true) {
    return concat(['static', space, printed]);
  } else {
    return printed;
  }
}
