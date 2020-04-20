/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, Binding} from '@romejs/js-compiler';
import inheritLoc from './inheritLoc';
import {
  AnyNode,
  exportLocalSpecifier,
  exportLocalDeclaration,
  identifier,
  referenceIdentifier,
} from '@romejs/js-ast';
import getBindingIdentifiers from './getBindingIdentifiers';
import isVariableIdentifier from './isVariableIdentifier';
import assertSingleOrMultipleNodes from './assertSingleOrMultipleNodes';
import {AnyVariableIdentifier} from '@romejs/js-ast/unions';

// This methods allows either passing in Bindings that could be present within deep scopes,
// or local names for the scope in the passed Path
export default function renameBindings(
  path: Path,
  oldToNewMapping: Map<Binding | string, string>,
): AnyNode | Array<AnyNode> {
  if (oldToNewMapping.size === 0) {
    return path.node;
  }

  const oldBindingToNewName: Map<Binding, string> = new Map();

  // get a list of the current bindings for this scope
  const oldNameToBinding: Map<string, undefined | Binding> = new Map();
  for (const [oldName, newName] of oldToNewMapping) {
    if (typeof oldName === 'string') {
      const binding = path.scope.getBinding(oldName);
      oldNameToBinding.set(oldName, binding);
    } else {
      oldBindingToNewName.set(oldName, newName);
    }
  }

  // discover nodes to replace first without manipulating the AST as that will change the scope and binding objects
  const replaceNodesWithName: Map<AnyVariableIdentifier, string> = new Map();
  path.traverse('renameBindingsCollector', (path) => {
    const {node, scope} = path;
    if (!isVariableIdentifier(node)) {
      return;
    }

    const binding = scope.getBinding(node.name);

    // oldName -> newName
    if (oldToNewMapping.has(node.name) && binding === oldNameToBinding.get(
        node.name,
      )) {
      const newName = oldToNewMapping.get(node.name);
      if (newName === undefined) {
        throw new Error('Should exist');
      }
      replaceNodesWithName.set(node, newName);
    }

    // Binding -> newName
    if (binding !== undefined && oldBindingToNewName.has(binding)) {
      const newName = oldBindingToNewName.get(binding);
      if (newName === undefined) {
        throw new Error('Should exist');
      }
      replaceNodesWithName.set(node, newName);
    }
  });
  if (replaceNodesWithName.size === 0) {
    return path.node;
  }

  //
  const replaced: Set<AnyNode> = new Set();

  // replace the nodes
  const renamedNode = path.reduce({
    name: 'renameBindings',
    enter(path): AnyNode | Array<AnyNode> {
      const {node} = path;

      // Retain the correct exported name for `export function` and `export class`
      if (node.type === 'ExportLocalDeclaration' && node.declaration !==
          undefined && (node.declaration.type === 'FunctionDeclaration' ||
            node.declaration.type ===
            'ClassDeclaration')) {
        const newName = replaceNodesWithName.get(node.declaration.id);

        if (newName !== undefined) {
          replaced.add(node.declaration.id);

          const oldName = node.declaration.id.name;

          return ([
            node.declaration,
            exportLocalDeclaration.create({
              specifiers: [
                exportLocalSpecifier.create({
                  loc: node.declaration.id.loc,
                  local: referenceIdentifier.quick(newName),
                  exported: identifier.quick(oldName),
                }),
              ],
            }),
          ] as Array<AnyNode>);
        }
      }

      // Retain the correct exported names for `export const`
      if (node.type === 'ExportLocalDeclaration' && node.declaration !==
          undefined) {
        const bindings = getBindingIdentifiers(node.declaration);
        let includesAny = false;
        for (const node of bindings) {
          if (replaceNodesWithName.has(node)) {
            includesAny = true;
            break;
          }
        }

        if (includesAny) {
          return ([
            node.declaration,
            exportLocalDeclaration.create({
              specifiers: bindings.map((node) => {
                let local: string = node.name;

                const newName = replaceNodesWithName.get(node);
                if (newName !== undefined) {
                  local = newName;
                  replaced.add(node);
                }

                return exportLocalSpecifier.create({
                  loc: node.loc,
                  local: referenceIdentifier.quick(local),
                  exported: identifier.quick(node.name),
                });
              }),
            }),
          ] as Array<AnyNode>);
        }
      }

      if (isVariableIdentifier(node)) {
        const newName = replaceNodesWithName.get(node);
        if (newName !== undefined) {
          replaced.add(node);
          return {
            ...node,
            name: newName,
            loc: inheritLoc(node, node.name),
          };
        }
      }

      return node;
    },
  }, {
    noScopeCreation: true,
  });

  //
  if (replaced.size !== replaceNodesWithName.size) {
    console.log({replaced, replaceNodesWithName});
    throw new Error('Missed some bindings');
  }

  return assertSingleOrMultipleNodes(renamedNode);
}
