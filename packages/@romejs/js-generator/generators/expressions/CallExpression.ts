/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {CallExpression, callExpression, AnyNode} from '@romejs/js-ast';

export default function CallExpression(generator: Generator, node: AnyNode) {
  node = callExpression.assert(node);

  const {callee} = node;
  generator.print(callee, node);

  let isMultiLine = false;
  const firstArg = node.arguments[0];
  if (callee && callee.loc && firstArg && firstArg.loc) {
    isMultiLine = firstArg.loc.start.line > callee.loc.end.line;
  }

  generator.print(node.typeArguments, node);
  generator.token('(');
  generator.printCommaList(node.arguments, node, {
    statement: isMultiLine,
  });
  generator.token(')');
}
