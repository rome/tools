/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {RegExpLiteral, regExpLiteral, AnyNode} from '@romejs/js-ast';

export default function RegExpLiteral(generator: Generator, node: AnyNode) {
  node = regExpLiteral.assert(node);

  const flags: Array<string> = [];

  if (node.global === true) {
    flags.push('g');
  }

  if (node.multiline === true) {
    flags.push('m');
  }

  if (node.sticky === true) {
    flags.push('y');
  }

  if (node.insensitive === true) {
    flags.push('i');
  }

  if (node.noDotNewline === true) {
    flags.push('s');
  }

  if (node.unicode === true) {
    flags.push('u');
  }

  generator.token(`/`);
  generator.print(node.expression, node);
  generator.token(`/${flags.join('')}`);
}
