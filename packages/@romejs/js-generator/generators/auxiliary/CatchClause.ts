/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {CatchClause, catchClause, AnyNode} from '@romejs/js-ast';

export default function CatchClause(generator: Generator, node: AnyNode) {
  node = catchClause.assert(node);
  catchClause.assert(node);
  generator.word('catch');
  generator.space();
  generator.token('(');
  generator.print(node.param, node);
  generator.token(')');
  generator.space();
  generator.print(node.body, node);
}
