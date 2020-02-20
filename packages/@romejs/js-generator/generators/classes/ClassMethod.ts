/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ClassMethod, classMethod, AnyNode} from '@romejs/js-ast';
import {printMethod} from '../utils';
export default function ClassMethod(generator: Generator, node: AnyNode) {
  node = classMethod.assert(node);
  classMethod.assert(node);
  if (node.meta.static === true) {
    generator.word('static');
    generator.space();
  }

  printMethod(generator, node);
}
