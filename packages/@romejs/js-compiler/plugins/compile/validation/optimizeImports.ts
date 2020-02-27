/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {
  AnyNode,
  ImportDeclaration,
  jsxIdentifier,
  bindingIdentifier,
  referenceIdentifier,
  AnyImportSpecifier,
  identifier,
  importDeclaration,
  importSpecifier,
  importSpecifierLocal,
} from '@romejs/js-ast';
import {Binding} from '@romejs/js-compiler';
import {isIdentifierish} from '@romejs/js-ast-utils';
import {TransformExitResult} from '@romejs/js-compiler';

// Eliminate this blacklist. This contains React for the following reason:
//   A user may write: import * as React from 'react';
//   We will remove the namespace and have only the used specifiers
//   But the JSX plugin inserts `React.createElement`. Oh no.
const BLACKLIST = ['React', 'react'];

function getName(node: AnyNode): undefined | string {
  if (node.type !== 'MemberExpression' && node.type !== 'JSXMemberExpression') {
    return;
  }

  const {property} = node;

  if (property.type === 'ComputedMemberProperty') {
    if (property.value.type === 'StringLiteral') {
      return property.value.value;
    }
  } else {
    if (isIdentifierish(property)) {
      return property.name;
    }
  }
}

export default {
  name: 'optimizeImports',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (node.type !== 'Program') {
      return node;
    }

    // Check if we have any wildcard imports
    const wildcardImports: Map<
      string,
      {
        binding: Binding;
        names: Set<string>;
        mappings: Map<string, string>;
        references: Set<AnyNode>;
      }
    > = new Map();
    const wildcardImportNodeToLocal: Map<ImportDeclaration, string> = new Map();
    for (const child of node.body) {
      if (
        child.type === 'ImportDeclaration' &&
        !BLACKLIST.includes(child.source.value) &&
        child.specifiers !== undefined
      ) {
        for (const specifier of child.specifiers) {
          if (specifier.type === 'ImportNamespaceSpecifier') {
            wildcardImports.set(specifier.local.name.name, {
              binding: path.scope.getBindingAssert(specifier.local.name.name),
              names: new Set(),
              mappings: new Map(),
              references: new Set(),
            });
            wildcardImportNodeToLocal.set(child, specifier.local.name.name);
          }
        }
      }
    }
    if (wildcardImports.size === 0) {
      return node;
    }

    // - Find all imported names from this namespace
    // - Remove the namespaces that have computed property access
    path.traverse('optimizeImportsWildcardCollector', path => {
      const {node, parent} = path;
      if (node.type !== 'ReferenceIdentifier') {
        return;
      }

      // Ensure we're referencing a wildcard import
      const wildcardInfo = wildcardImports.get(node.name);
      if (wildcardInfo === undefined) {
        return;
      }

      // Ensure that the binding hasn't been shadowed
      if (path.scope.getBinding(node.name) !== wildcardInfo.binding) {
        return;
      }

      const isComputed =
        parent.type === 'MemberExpression' &&
        parent.object === node &&
        getName(parent) === undefined;
      const isUnboxed =
        parent.type !== 'MemberExpression' &&
        parent.type !== 'JSXMemberExpression';

      if (isComputed || isUnboxed) {
        // Deopt as we can't follow this
        wildcardImports.delete(node.name);
      } else {
        const name = getName(parent);
        if (name === undefined) {
          throw new Error('Expected name');
        }
        wildcardInfo.names.add(name);
        wildcardInfo.references.add(parent);
      }
    });
    if (wildcardImports.size === 0) {
      return node;
    }

    // Populate the `mappings` field with a uid
    for (const info of wildcardImports.values()) {
      for (const name of info.names) {
        info.mappings.set(name, path.scope.generateUid(name));
      }
    }

    return path.reduce({
      name: 'optimizeImportWilcards',
      enter(path): AnyNode {
        const {node} = path;

        // Replace all member expressions with their uids
        if (
          (node.type === 'MemberExpression' ||
            node.type === 'JSXMemberExpression') &&
          isIdentifierish(node.object)
        ) {
          const wildcardInfo = wildcardImports.get(node.object.name);
          if (wildcardInfo !== undefined && wildcardInfo.references.has(node)) {
            const name = getName(node);
            if (name === undefined) {
              throw new Error('Expected name');
            }

            const newName = wildcardInfo.mappings.get(name);
            if (newName === undefined) {
              throw new Error('Expected newName');
            }

            if (node.type === 'JSXMemberExpression') {
              return jsxIdentifier.quick(newName);
            } else {
              return referenceIdentifier.quick(newName);
            }
          }
        }

        // Add new specifiers to wildcard import declarations
        if (
          node.type === 'ImportDeclaration' &&
          wildcardImportNodeToLocal.has(node)
        ) {
          const local = wildcardImportNodeToLocal.get(node);
          if (local === undefined) {
            throw new Error('Expected local');
          }

          const wildcardInfo = wildcardImports.get(local);
          if (wildcardInfo === undefined) {
            // We would have deopted earlier
            return node;
          }

          // Remove wildcard specifier
          let specifiers: ImportDeclaration['specifiers'] = [];
          if (node.specifiers !== undefined) {
            specifiers = node.specifiers.filter(
              (specifier: AnyImportSpecifier) => {
                if (
                  specifier.type === 'ImportNamespaceSpecifier' &&
                  specifier.local.name.name === local
                ) {
                  return false;
                } else {
                  return true;
                }
              },
            );
          }

          // Add on our new mappings
          for (const [imported, local] of wildcardInfo.mappings) {
            specifiers.push(
              importSpecifier.create({
                imported: identifier.quick(imported),
                local: importSpecifierLocal.quick(
                  bindingIdentifier.quick(local),
                ),
              }),
            );
          }

          return importDeclaration.create({specifiers, source: node.source});
        }

        return node;
      },
    });
  },
};
