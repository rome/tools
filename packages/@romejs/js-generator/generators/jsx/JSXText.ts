/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {JSXText, jsxText, AnyNode} from '@romejs/js-ast';
import {escapeXHTMLEntities} from '@romejs/js-parser';

export default function JSXText(generator: Generator, node: AnyNode): Tokens {
  node = jsxText.assert(node);
  return [operator(escapeXHTMLEntities(node.value))];
}
