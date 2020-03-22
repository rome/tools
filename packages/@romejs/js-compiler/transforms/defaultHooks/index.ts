/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createHook, Path} from '@romejs/js-compiler';
import {
  ReferenceIdentifier,
  variableDeclaration,
  AnyExpression,
  variableDeclarator,
  referenceIdentifier,
  bindingIdentifier,
  assignmentIdentifier,
  AssignmentIdentifier,
  AnyNode,
  variableDeclarationStatement,
} from '@romejs/js-ast';

type VariableInjectorState = {bindings: Array<[string, undefined | AnyExpression]>};

type VariableInjectorArgs = {
  name?: string;
  init?: AnyExpression;
};

export const bindingInjector = createHook<
  VariableInjectorState,
  VariableInjectorArgs,
  [ReferenceIdentifier, AssignmentIdentifier]
>({
  name: 'bindingInjectorHook',

  initialState: {
    bindings: [],
  },

  call(path: Path, state: VariableInjectorState, opts: VariableInjectorArgs = {}) {
    const name = opts.name === undefined ? path.scope.generateUid() : opts.name;

    const ref = referenceIdentifier.quick(name);

    // lol
    const ass = assignmentIdentifier.quick(name);

    return {
      value: [ref, ass],
      state: {
        bindings: [...state.bindings, [name, opts.init]],
      },
    };
  },

  exit(path: Path, state: VariableInjectorState): AnyNode {
    const {node} = path;

    if (node.type !== 'BlockStatement' && node.type !== 'Program') {
      throw new Error('Never should have been used as a provider');
    }

    const {bindings} = state;
    if (bindings.length === 0) {
      return node;
    }

    return {
      ...node,
      body: [
        variableDeclarationStatement.quick(variableDeclaration.create({
          kind: 'var',
          declarations: bindings.map(([name, init]) => {
            return variableDeclarator.create({
              id: bindingIdentifier.quick(name),
              init,
            });
          }),
        })),

        ...node.body,
      ],
    };
  },
});

export const variableInjectorVisitor = {
  name: 'variableInjector',
  enter(path: Path) {
    const {node} = path;

    if (node.type === 'BlockStatement' || node.type === 'Program') {
      path.provideHook(bindingInjector);
    }

    return node;
  },
};
