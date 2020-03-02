/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {ObjectProperty, ObjectMethod, SpreadProperty} from '@romejs/js-ast';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {markup} from '@romejs/string-markup';

function extractPropertyKey(
  node: ObjectProperty | ObjectMethod | SpreadProperty,
) {
  if (
    (node.type === 'ObjectMethod' || node.type === 'ObjectProperty') &&
    node.key.type === 'StaticPropertyKey'
  ) {
    const {value} = node.key;

    if (value.type === 'PrivateName') {
      return value.id.name;
    }

    if (value.type === 'Identifier') {
      return value.name;
    }

    return value.value;
  }

  return null;
}

export default {
  name: 'noDuplicateKeys',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (node.type === 'ObjectExpression') {
      const previousKeys = new Set();

      for (const prop of node.properties) {
        const key = extractPropertyKey(prop);

        if (key !== null) {
          if (previousKeys.has(key)) {
            path.context.addNodeDiagnostic(prop, {
              category: 'lint/noDuplicateKeys',
              message: markup`Duplicate key <emphasis>${key}</emphasis>`,
            });
          }

          previousKeys.add(key);
        }
      }
    }

    return node;
  },
};
