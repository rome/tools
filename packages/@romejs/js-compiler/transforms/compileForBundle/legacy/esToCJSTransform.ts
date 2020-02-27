/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  AnyStatement,
  FunctionExpression,
  program,
  stringLiteral,
  ClassExpression,
} from '@romejs/js-ast';
import {template, getBindingIdentifiers} from '@romejs/js-ast-utils';
import {getOptions, getModuleId} from '../_utils';
import {Path, FunctionBinding} from '@romejs/js-compiler';

export default {
  name: 'esToCJSTransform',
  enter(path: Path): AnyNode {
    const {node} = path;
    if (!program.is(node)) {
      return node;
    }

    const options = getOptions(path.context);

    const topBody: Array<AnyStatement> = [];
    const bottomBody: Array<AnyStatement> = [];

    for (const bodyNode of node.body) {
      if (bodyNode.type === 'ImportDeclaration') {
        if (bodyNode.importKind === 'type') {
          continue;
        }

        const moduleId = getModuleId(bodyNode.source.value, options);
        if (moduleId === undefined) {
          continue;
        }

        const source = stringLiteral.create({
          value: moduleId,
        });

        const {specifiers} = bodyNode;
        if (specifiers === undefined || specifiers.length === 0) {
          topBody.push(template.statement`Rome.requireNamespace(${source});`);
        } else {
          for (const specifier of specifiers) {
            if (specifier.local.importKind === 'type') {
              continue;
            }

            if (specifier.type === 'ImportSpecifier') {
              topBody.push(
                template.statement`const ${specifier.local.name} = Rome.requireNamespace(${source}).${specifier.imported};`,
              );
            } else if (specifier.type === 'ImportNamespaceSpecifier') {
              topBody.push(
                template.statement`const ${specifier.local.name} = Rome.requireNamespace(${source});`,
              );
            } else if (specifier.type === 'ImportDefaultSpecifier') {
              topBody.push(
                template.statement`const ${specifier.local.name} = Rome.requireDefault(${source});`,
              );
            }
          }
        }
        continue;
      }

      if (bodyNode.type === 'ExportAllDeclaration') {
        // TODO
        continue;
      }

      if (bodyNode.type === 'ExportNamedDeclaration') {
        // Ignore typed exports
        if (bodyNode.exportKind === 'type') {
          continue;
        }

        const {declaration, specifiers, source} = bodyNode;

        if (declaration !== undefined) {
          // Hoist function declarations
          if (declaration.type === 'FunctionDeclaration') {
            topBody.push(
              template.statement`exports.${declaration.id} = ${declaration.id}`,
            );
            bottomBody.push(declaration);
            continue;
          }

          // Handle type declarations (these have no runtime ordering implications)
          if (
            declaration.type === 'TSModuleDeclaration' ||
            declaration.type === 'TSEnumDeclaration' ||
            declaration.type === 'FlowInterfaceDeclaration' ||
            declaration.type === 'TypeAliasTypeAnnotation' ||
            declaration.type === 'TSInterfaceDeclaration' ||
            declaration.type === 'TSDeclareFunction' ||
            declaration.type === 'FlowOpaqueType'
          ) {
            bottomBody.push(declaration);
            continue;
          }

          // Handle variables and classes
          if (
            declaration.type === 'VariableDeclarationStatement' ||
            declaration.type === 'ClassDeclaration'
          ) {
            bottomBody.push(declaration);

            for (const id of getBindingIdentifiers(declaration)) {
              topBody.push(template.statement`exports.${id} = undefined;`);
              bottomBody.push(template.statement`exports.${id} = ${id};`);
            }
          }
        }

        if (specifiers !== undefined) {
          for (const specifier of specifiers) {
            if (specifier.type === 'ExportDefaultSpecifier') {
              // TODO only allowed for `source`
            }

            if (specifier.type === 'ExportNamespaceSpecifier') {
              // TODO only allowed for `source`
            }

            // TODO skip type exports
            if (specifier.type === 'ExportSpecifier') {
              if (source === undefined) {
                const binding = path.scope.getBinding(specifier.local.name);

                if (binding instanceof FunctionBinding) {
                  topBody.push(
                    template.statement`exports.${specifier.exported} = ${specifier.local};`,
                  );
                } else {
                  topBody.push(
                    template.statement`exports.${specifier.exported} = undefined;`,
                  );
                  bottomBody.push(
                    template.statement`exports.${specifier.exported} = ${specifier.local};`,
                  );
                }
              } else {
                topBody.push(
                  template.statement`Object.defineProperty(exports, ${stringLiteral.create(
                    {
                      value: specifier.exported.name,
                    },
                  )}, {
                    get: function() {
                      return Rome.requireNamespace(${source}).${
                    specifier.local
                  };
                    },
                  })`,
                );
              }
            }
          }
        }

        continue;
      }

      if (bodyNode.type === 'ExportDefaultDeclaration') {
        const {declaration} = bodyNode;

        // Hoist function declarations
        if (declaration.type === 'FunctionDeclaration') {
          // If it has an id then there's no way that anything in the program can refer to it, so inline it as a function expression
          if (declaration.id === undefined) {
            const expr: FunctionExpression = {
              ...declaration,
              type: 'FunctionExpression',
            };
            topBody.push(template.statement`exports.default = ${expr};`);
          } else {
            topBody.push(declaration);
            topBody.push(
              template.statement`exports.default = ${declaration.id};`,
            );
          }
          continue;
        }

        // Handle classes
        if (declaration.type === 'ClassDeclaration') {
          // Technically we could hoist these if they have no super class, but we don't as it's not spec compliant
          topBody.push(template.statement`exports.default = undefined;`);
          if (declaration.id === undefined) {
            const expr: ClassExpression = {
              ...declaration,
              type: 'ClassExpression',
            };
            bottomBody.push(template.statement`exports.default = ${expr};`);
          } else {
            bottomBody.push(declaration);
            bottomBody.push(
              template.statement`exports.default = ${declaration.id};`,
            );
          }
          continue;
        }

        // Handle type declarations (these have no runtime ordering implications)
        if (
          declaration.type === 'FlowDeclareOpaqueType' ||
          declaration.type === 'TSInterfaceDeclaration' ||
          declaration.type === 'TSDeclareFunction'
        ) {
          // Maybe we should keep them? Not sure what they would desugar to
          continue;
        }

        // Otherwise it's an expression
        bottomBody.push(template.statement`exports.default = ${declaration};`);

        // There are cases where we could omit this declaration at all if we the file has no imports, some other conditions etc
        topBody.push(template.statement`exports.default = undefined;`);

        continue;
      }

      bottomBody.push(bodyNode);
    }

    return {
      ...node,
      body: [...topBody, ...bottomBody],
    };
  },
};
