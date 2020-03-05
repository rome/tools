/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path, Scope, createHook} from '@romejs/js-compiler';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';
import {Dict} from '@romejs/typescript-helpers';
import {ArgumentsBinding} from '@romejs/js-compiler/scope/bindings';

type State = {
  usedBindings: Dict<boolean>;
  scope: undefined | Scope;
};

const initialState: State = {
  usedBindings: {},
  scope: undefined,
};

const provider = createHook<State, undefined, AnyNode>({
  name: 'unusedVariablesProvider',

  initialState,

  call(path: Path, state: State) {
    const {node} = path;
    if (
      node.type !== 'ReferenceIdentifier' &&
      node.type !== 'JSXReferenceIdentifier'
    ) {
      throw new Error('Expected only Identifier to be dispatched');
    }

    const binding = path.scope.getBinding(node.name);

    // Check if this binding belongs to the scope we're tracking
    if (binding === undefined || binding.scope !== state.scope) {
      return {
        bubble: true,
        value: node,
        state,
      };
    }

    // Mark this binding as used
    return {
      value: node,
      state: {
        ...state,
        usedBindings: {
          ...state.usedBindings,
          [node.name]: true,
        },
      },
    };
  },

  exit(path, state) {
    for (const name in state.usedBindings) {
      const used = state.usedBindings[name];
      const binding = path.scope.getBinding(name);

      if (used === false && binding !== undefined) {
        path.context.addNodeDiagnostic(binding.node, {
          category: 'lint/unusedVariables',
          message: `Unused ${binding.kind} <emphasis>${name}</emphasis>`,
        });
      }
    }

    return path.node;
  },
});

export default {
  name: 'unusedVariables',
  enter(path: Path): AnyNode {
    const {node, scope} = path;

    if (scope.node === node) {
      const usedBindings: Dict<boolean> = {};

      // Get all the non-exported bindings in this file and mark them as unused
      for (const [name, binding] of scope.getOwnBindings()) {
        if (binding instanceof ArgumentsBinding) {
          continue;
        }

        if (binding.isExported) {
          continue;
        }

        usedBindings[name] = false;
      }

      // For functions, consider all parameters except the last to be used
      if (
        node.type === 'FunctionDeclaration' ||
        node.type === 'FunctionExpression' ||
        node.type === 'ObjectMethod' ||
        node.type === 'ClassMethod' ||
        node.type === 'ArrowFunctionExpression'
      ) {
        for (const {name} of getBindingIdentifiers(
          node.head.params.slice(0, -1),
        )) {
          usedBindings[name] = true;
        }

        // For functions that have a single throw statement in the body, consider all their arguments
        // to be used as this is typically an interface definition
        const {body: block} = node;
        if (
          block.type === 'BlockStatement' &&
          block.body.length === 1 &&
          block.body[0].type === 'ThrowStatement'
        ) {
          for (const {name} of getBindingIdentifiers(node.head.params)) {
            usedBindings[name] = true;
          }
        }
      }

      // For a named function expression, don't consider the id to be unused
      if (node.type === 'FunctionExpression' && node.id !== undefined) {
        usedBindings[node.id.name] = true;
      }

      return path.provideHook(provider, {
        usedBindings,
        scope,
      });
    }

    if (
      node.type === 'JSXReferenceIdentifier' ||
      node.type === 'ReferenceIdentifier'
    ) {
      return path.callHook(provider, undefined);
    }

    return node;
  },
};
