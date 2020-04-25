/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {ObjectMethod, ObjectProperty, SpreadProperty} from '@romejs/js-ast';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {descriptions} from '@romejs/diagnostics';
import {DiagnosticsDuplicateHelper} from '../../lib/DiagnosticsDuplicateHelper';

function extractPropertyKey(
  node: ObjectProperty | ObjectMethod | SpreadProperty,
): string | undefined {
  if ((node.type === 'ObjectMethod' || node.type === 'ObjectProperty') &&
        node.key.type ===
        'StaticPropertyKey') {
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
    const {node, context} = path;

    if (node.type === 'ObjectExpression') {
      const duplicates = new DiagnosticsDuplicateHelper(
        context,
        descriptions.LINT.NO_DUPLICATE_KEYS,
      );

      for (const prop of node.properties) {
        const key = extractPropertyKey(prop);
        if (key !== undefined) {
          duplicates.addLocation(key, prop.loc);
        }
      }

      duplicates.process();
    }

    return node;
  },
};
