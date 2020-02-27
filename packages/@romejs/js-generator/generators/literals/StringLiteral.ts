/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {escapeString} from '@romejs/string-escape';
import {StringLiteral, stringLiteral, AnyNode} from '@romejs/js-ast';
import {escapeXHTMLEntities} from '@romejs/js-parser';
export default function StringLiteral(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node =
    node.type == 'StringLiteralTypeAnnotation' || node.type === 'Directive'
      ? node
      : stringLiteral.assert(node);

  // JSX Attribute strings have ridiculous alternate semantics, should probably be a distinct AST node
  const quotes = parent.type === 'JSXAttribute' ? '"' : "'";
  const value =
    parent.type === 'JSXAttribute'
      ? escapeXHTMLEntities(node.value)
      : node.value;

  generator.token(
    escapeString(value, {
      quote: quotes,
    }),
  );
}
