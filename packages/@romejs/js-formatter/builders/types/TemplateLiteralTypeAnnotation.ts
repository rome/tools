/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TemplateLiteralTypeAnnotation} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token} from '../../tokens';
import {escapeString} from '@romejs/string-escape';

export default function TemplateLiteralTypeAnnotation(
  builder: Builder,
  node: TemplateLiteralTypeAnnotation,
): Token {
  return escapeString(node.value, {quote: '`'});
}
