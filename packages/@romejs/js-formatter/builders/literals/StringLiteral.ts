/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {escapeString} from '@romejs/string-escape';
import {StringLiteral, stringLiteral, AnyNode} from '@romejs/js-ast';
import {escapeXHTMLEntities} from '@romejs/js-parser';
import {operator} from '@romejs/js-formatter/tokens';

export default function StringLiteral(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = node.type === 'StringLiteralTypeAnnotation' || node.type ===
    'Directive' ? node : stringLiteral.assert(node);

  // JSX Attribute strings have ridiculous alternate semantics, should probably be a distinct AST node
  const quotes = parent.type === 'JSXAttribute' || node.value.includes("'")
    ? '"'
    : "'";

  const value = parent.type === 'JSXAttribute'
    ? escapeXHTMLEntities(node.value)
    : node.value;

  return [
    operator(escapeString(value, {
      quote: quotes,
    })),
  ];
}
