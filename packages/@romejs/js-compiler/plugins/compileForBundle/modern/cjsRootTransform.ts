/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {getOptions, getPrefixedNamespace, getPrivateName} from '../_utils';
import {renameBindings} from '@romejs/js-ast-utils';

export default {
  name: 'cjsRootTransform',
  enter(path: Path) {
    const {node, scope, context} = path;

    const {moduleId} = getOptions(context);

    if (node.type === 'Program') {
      const mappings = new Map();

      // make all variables private
      for (const [name] of path.scope.bindings) {
        mappings.set(name, getPrivateName(name, moduleId));
      }

      if (scope.hasBinding('exports') === false) {
        mappings.set('exports', getPrefixedNamespace(moduleId));
      }

      const newProgram = renameBindings(path, mappings);
      return newProgram;
    }

    return node;
  },
};
