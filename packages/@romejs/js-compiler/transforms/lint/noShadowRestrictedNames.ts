/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {builtin, es5, es2015, es2017} from '@romejs/js-compiler/scope/globals';
import {markup} from '@romejs/string-markup';

const restrictedNames = new Set([...builtin, ...es5, ...es2015, ...es2017]);

export default {
  name: 'noShadowRestrictedNames',
  enter(path: Path): TransformExitResult {
    const {node, context, scope} = path;

    if (scope.node === node) {
      for (const [name, binding] of scope.getOwnBindings()) {
        if (restrictedNames.has(name)) {
          context.addNodeDiagnostic(binding.node, {
            category: 'lint/noShadowRestrictedNames',
            message: markup`Shadowing of global property <emphasis>${name}</emphasis>`,
            advice: [
              {
                type: 'log',
                category: 'info',
                message:
                  "Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.",
              },
            ],
          });
        }
      }
    }

    return node;
  },
};
