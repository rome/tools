/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createHook} from '@romejs/js-compiler';
import {
  AnyComment,
  AnyCommentOptionalId,
  AnyExpression,
  AnyNode,
  AssignmentIdentifier,
  ReferenceIdentifier,
  assignmentIdentifier,
  bindingIdentifier,
  referenceIdentifier,
  variableDeclaration,
  variableDeclarationStatement,
  variableDeclarator,
} from '@romejs/js-ast';

type VariableInjectorState = {
  bindings: Array<[string, undefined | AnyExpression]>;
};

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

  call(
    path: Path,
    state: VariableInjectorState,
    opts: VariableInjectorArgs = {},
  ) {
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

type CommentInjectorState = {
  comments: Array<AnyComment>;
};

type CommentInjectorArg = AnyCommentOptionalId;

export const commentInjector = createHook<
  CommentInjectorState,
  CommentInjectorArg,
  string
>({
  name: 'commentInjectorHook',

  initialState: {
    comments: [],
  },

  call(path: Path, state: CommentInjectorState, comment: CommentInjectorArg) {
    let commentWithId: AnyComment;
    let comments = state.comments;

    const {id} = comment;
    if (id === undefined) {
      commentWithId = path.context.comments.addComment(comment);
    } else {
      // This comment already has an id so update it
      commentWithId = {
        ...comment,
        id,
      };
      path.context.comments.updateComment(commentWithId);

      // Remove from existing comments
      comments = comments.filter((comment) => comment.id !== id);
    }

    return {
      value: commentWithId.id,
      state: {
        comments: [...comments, commentWithId],
      },
    };
  },

  exit(path: Path, state: CommentInjectorState): AnyNode {
    const {node} = path;

    if (node.type !== 'Program') {
      throw new Error('Never should have been used as a provider');
    }

    return {
      ...node,
      comments: [...node.comments, ...state.comments],
    };
  },
});

export const commentInjectorVisitor = {
  name: 'commentInjector',
  enter(path: Path) {
    const {node, context} = path;

    if (node.type === 'CommentBlock' || node.type === 'CommentLine') {
      context.comments.updateComment(node);
    }

    if (node.type === 'Program') {
      context.comments.setComments(node.comments);
      return path.provideHook(commentInjector);
    }

    return node;
  },
};
