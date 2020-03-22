/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noLabelVar',
  enter(path: Path): AnyNode {
    const {node, scope} = path;

    if (node.type === 'LabeledStatement') {
      const name = node.label.name;
      const binding = scope.getBinding(name);
      const isDefined = binding !== undefined || scope.getRootScope().isGlobal(
        name,
      );

      if (isDefined) {
        path.context.addNodeDiagnostic(node, {
          description: descriptions.LINT.NO_LABEL_VAR,
        });
      }
    }

    return node;
  },
};
