/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createHook} from '@romejs/js-compiler';
import {
  variableDeclaration,
  thisExpression,
  variableDeclarator,
  identifier,
  Identifier,
  ThisExpression,
  bindingIdentifier,
  AnyNode,
  variableDeclarationStatement,
} from '@romejs/js-ast';
import {inheritLoc} from '@romejs/js-ast-utils';

function isInsideArrow(path: Path): boolean {
  for (const ancestor of path.ancestryPaths) {
    const {type} = ancestor.node;

    // If we hit a function first then it takes precedence over any arrow
    // NOTE: There are other nodes for functions not included
    if (type === 'FunctionExpression' || type === 'FunctionDeclaration') {
      return false;
    }

    if (type === 'ArrowFunctionExpression') {
      return true;
    }
  }

  return false;
}

type State = {
  id: undefined | string;
};

const arrowProvider = createHook<State, ThisExpression, Identifier>({
  name: 'arrowProvider',

  initialState: {
    id: undefined,
  },

  call(
    path: Path,
    state: State,
    node: ThisExpression,
  ): {value: Identifier; state: State} {
    const id = state.id === undefined ? path.scope.generateUid() : state.id;
    return {
      value: identifier.create({
        name: id,
        loc: inheritLoc(node, 'this'),
      }),
      state: {
        id,
      },
    };
  },

  exit(path: Path, state: State): AnyNode {
    const {node} = path;

    if (
      node.type !== 'FunctionDeclaration' &&
      node.type !== 'FunctionExpression'
    ) {
      throw new Error('Only ever expected function nodes');
    }

    // This is called after the subtree has been transformed
    if (state.id === undefined) {
      // No `ThisExpression`s were rewritten
      return node;
    } else {
      // Inject the binding into the function block
      return {
        ...node,
        body: {
          ...node.body,
          body: [
            variableDeclarationStatement.quick(
              variableDeclaration.create({
                kind: 'const',
                declarations: [
                  variableDeclarator.create({
                    id: bindingIdentifier.quick(state.id),
                    init: thisExpression.create({}),
                  }),
                ],
              }),
            ),
            ...node.body.body,
          ],
        },
      };
    }
  },
});

export default {
  name: 'arrowFunctions',

  enter(path: Path) {
    const {node} = path;

    if (
      node.type === 'FunctionDeclaration' ||
      node.type === 'FunctionExpression'
    ) {
      // Add a provider to consume `this` inside of arrow functions
      return path.provideHook(arrowProvider);
    }

    if (node.type === 'ThisExpression' && isInsideArrow(path)) {
      // If we're a this expression and we're inside of an arrow then consume us by a descendent provider
      return path.callHook(arrowProvider, node);
    }

    return node;
  },

  exit(path: Path) {
    const {node} = path;

    if (node.type === 'ArrowFunctionExpression') {
      // Convert all arrow functions into normal functions, we do this in the `exit` method because we
      // still need the arrow to be in the tree for the `isInsideArrow` call in `enter to work
      return {
        ...node,
        type: 'FunctionExpression',
      };
    }

    return node;
  },
};
