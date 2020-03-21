/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {SequenceExpression, sequenceExpression, AnyNode} from '@romejs/js-ast';

export default function SequenceExpression(generator: Generator, node: AnyNode) {
  node = sequenceExpression.assert(node);

  generator.printCommaList(node.expressions, node);
}
