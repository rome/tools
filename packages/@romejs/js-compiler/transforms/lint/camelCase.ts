/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from '@romejs/js-compiler';
import {toCamelCase} from '@romejs/string-utils';
import {Binding} from '@romejs/js-compiler/scope/bindings';
import {descriptions} from '@romejs/diagnostics';
import {
  renameBindings,
  isVariableIdentifier,
  isIdentifierish,
} from '@romejs/js-ast-utils';

// Allow prefixed underscores
function toVariableCamelCase(name: string): string {
  // Allow shouty constants
  if (name.toUpperCase() === name) {
    return name;
  }

  let prefix = '';
  let suffix = '';

  const prefixDashes = name.match(/^_+/);
  if (prefixDashes != null) {
    prefix = prefixDashes[0];
  }

  const suffixDashes = name.match(/_+$/);
  if (suffixDashes != null) {
    suffix = suffixDashes[0];
  }

  // Remove prefix and suffix
  let slicedName = name.slice(prefix.length);
  if (suffix.length > 0) {
    slicedName = name.slice(0, -suffix.length);
  }

  return prefix + toCamelCase(slicedName) + suffix;
}

export default {
  name: 'camelCase',
  enter(path: Path): TransformExitResult {
    const {node, scope, context} = path;

    // Check variables
    if (node === scope.node) {
      const renames: Map<Binding, string> = new Map();

      for (const [name, binding] of scope.getOwnBindings()) {
        const camelName = toVariableCamelCase(name);
        if (camelName !== name) {
          const {suppressed} = context.addNodeDiagnostic(
            binding.node,
            descriptions.LINT.VARIABLE_CAMEL_CASE(name, camelName),
            {fixable: true},
          );
          if (!suppressed) {
            renames.set(binding, camelName);
          }
        }
      }

      if (renames.size > 0) {
        return renameBindings(path, renames);
      }
    }

    // Check regular identifiers, variable identifiers have already been checked above
    if (isIdentifierish(node) && !isVariableIdentifier(node)) {
      const {name} = node;
      const camelName = toVariableCamelCase(name);
      if (camelName !== name) {
        return context.addFixableDiagnostic({
          old: node,
          fixed: {...node, name: camelName},
        }, descriptions.LINT.IDENTIFIER_CAMEL_CASE(name, camelName));
      }
    }

    return node;
  },
};
