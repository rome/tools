/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {ObjectProperty, ObjectMethod, SpreadProperty} from '@romejs/js-ast';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {descriptions} from '@romejs/diagnostics';

function extractPropertyKey(
  node: ObjectProperty | ObjectMethod | SpreadProperty,
): string | undefined {
  if ((node.type === 'ObjectMethod' || node.type === 'ObjectProperty') &&
    node.key.type === 'StaticPropertyKey') {
    const {value} = node.key;

    if (value.type === 'PrivateName') {
      return value.id.name;
    }

    if (value.type === 'Identifier') {
      return value.name;
    }

    return String(value.value);
  }

  return undefined;
}

export default {
  name: 'noDuplicateKeys',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (node.type === 'ObjectExpression') {
      const previousKeys = new Set();

      for (const prop of node.properties) {
        const key = extractPropertyKey(prop);

        if (key !== undefined) {
          if (previousKeys.has(key)) {
            path.context.addNodeDiagnostic(
              prop,
              descriptions.LINT.NO_DUPLICATE_KEYS(key),
            );
          }

          previousKeys.add(key);
        }
      }
    }

    return node;
  },
};
