/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {doesNodeMatchPattern} from '@romejs/js-ast-utils';
import {stringLiteral} from '@romejs/js-ast';

export default {
  name: 'inlineEnv',
  enter(path: Path) {
    const {node} = path;

    if (
      node.type === 'MemberExpression' &&
      node.property.value.type === 'Identifier' &&
      node.property.value.name === 'NODE_ENV' &&
      !path.scope.hasBinding('process') &&
      doesNodeMatchPattern(node, 'process.env.NODE_ENV')
    ) {
      return stringLiteral.create({
        value: 'development',
      });
    }

    return node;
  },
};
