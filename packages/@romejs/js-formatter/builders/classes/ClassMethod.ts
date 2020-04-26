/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {ClassMethod} from '@romejs/js-ast';
import {printMethod} from '../utils';

export default function ClassMethod(builder: Builder, node: ClassMethod): Token {
  const printed = printMethod(builder, node);

  if (node.meta.static === true) {
    return concat(['static', space, printed]);
  } else {
    return printed;
  }
}
