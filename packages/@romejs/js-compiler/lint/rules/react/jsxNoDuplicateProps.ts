/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from '@romejs/diagnostics';
import {Path, TransformExitResult} from '@romejs/js-compiler';
import { JSXIdentifier } from '@romejs/js-ast';
import { DiagnosticsDuplicateHelper } from '../../../lib/DiagnosticsDuplicateHelper';

function getAttributeKey(name: JSXIdentifier | string): string {
  if (typeof name === 'string') {
    return name;
  } else {
    return name.name;
  }
}

export default {
  name: 'jsxNoDuplicateProps',
  enter(path: Path): TransformExitResult {
    const {context, node} = path;
    if (node.type === 'JSXElement') {
      const duplicates = new DiagnosticsDuplicateHelper(
        context,
        descriptions.LINT.REACT_NO_DUPLICATE_PROPS,
      );

      for (const attr of node.attributes) {
        if (attr.type === 'JSXAttribute') {
          duplicates.addLocation(getAttributeKey(attr.name.name), attr.loc);
        }
      }
      duplicates.process();
    }

    return node;
  },
};
