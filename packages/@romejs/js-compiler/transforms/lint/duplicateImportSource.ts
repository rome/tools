/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, ImportDeclaration, AnyStatement} from '@romejs/js-ast';
import {SourceLocation} from '@romejs/parser-core';

export default {
  name: 'duplicateImport',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'Program') {
      const seenSources: Map<string, undefined | SourceLocation> = new Map();
      let shouldFix = false;

      for (const bodyNode of node.body) {
        if (bodyNode.type === 'ImportDeclaration') {
          const source = bodyNode.source.value;

          // Allow duplicate sources if the `importKind` is different
          const sourceKey = bodyNode.importKind === undefined
            ? source : `${bodyNode.importKind}:${source}`;

          const seenLoc = seenSources.get(sourceKey);
          if (seenLoc === undefined) {
            seenSources.set(sourceKey, bodyNode.loc);
          } else {
            shouldFix = true;
            path.context.addNodeDiagnostic(bodyNode, {
              fixable: true,
              category: 'lint/duplicateImportSource',
              message: 'This module has already been imported',
              advice: [
                {
                  type: 'log',
                  category: 'info',
                  message: 'Previously imported here',
                },
                {
                  type: 'frame',
                  ...seenLoc,
                },
              ],
            });
          }
        }
      }

      // Defer fixing unless it's totally necessary since there's additional overhead
      if (shouldFix) {
        const skipImports: Set<ImportDeclaration> = new Set();

        const newBody: Array<AnyStatement> = [];

        for (let i = 0; i < node.body.length; i++) {
          const bodyNode = node.body[i];

          if (bodyNode.type === 'ImportDeclaration') {
            // Skip import if it's already been consumed
            if (skipImports.has(bodyNode)) {
              continue;
            }

            let specifiers = bodyNode.specifiers === undefined
              ? [] : bodyNode.specifiers;

            // Find and concat all duplicate imports
            for (let x = i + 1; x < node.body.length; x++) {
              const possibleDuplicateNode = node.body[x];

              if (possibleDuplicateNode.type === 'ImportDeclaration' &&
                bodyNode.source.value === possibleDuplicateNode.source.value &&
                bodyNode.importKind === possibleDuplicateNode.importKind) {
                skipImports.add(possibleDuplicateNode);
                if (possibleDuplicateNode.specifiers !== undefined) {
                  specifiers = [
                    ...specifiers,
                    ...possibleDuplicateNode.specifiers,
                  ];
                }
              }
            }

            newBody.push({
              ...bodyNode,
              specifiers,
            });
          } else {
            newBody.push(bodyNode);
          }
        }
        return {
          ...node,
          body: newBody,
        };
      }
    }
    return node;
  },
};
