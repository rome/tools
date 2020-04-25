/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {isInTypeAnnotation} from '@romejs/js-ast-utils';
import {descriptions} from '@romejs/diagnostics';

const RESTRICTED_GLOBALS = ['event', 'error'];

export default {
  name: 'restrictedGlobal',
  enter(path: Path): AnyNode {
    const {node, scope} = path;

    if ((node.type === 'ReferenceIdentifier' || node.type ===
        'JSXReferenceIdentifier') && !isInTypeAnnotation(path)) {
      const {name} = node;
      const binding = scope.getBinding(name);

      const isDefined = binding !== undefined;
      const isAGlobal = scope.getRootScope().isGlobal(name);

      if (!isDefined && isAGlobal && RESTRICTED_GLOBALS.includes(name)) {
        path.context.addNodeDiagnostic(
          node,
          descriptions.LINT.RESTRICTED_GLOBALS(name),
        );
      }
    }

    return node;
  },
};
