/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, MOCK_PROGRAM} from '@romejs/js-ast';
import {DEFAULT_PROJECT_CONFIG} from '@romejs/project';
import {Context, Path, TransformVisitors} from '@romejs/js-compiler';
import {SourceLocation} from '@romejs/parser-core';
import {AnyComment} from '@romejs/js-ast/unions';
import {JSNodeBase} from '@romejs/js-ast/base';

function removeProp<T extends {loc?: SourceLocation}>(obj: T): Omit<T, 'loc'> {
  const {loc, ...locless} = obj;
  loc;
  return locless;
}

function removeComments(
  comments: undefined | Array<AnyComment>,
): undefined | Array<AnyComment> {
  if (comments === undefined) {
    return undefined;
  }

  const newComments: Array<AnyComment> = comments.map(
    (comment) => comment.type ===
      'CommentBlock' ? removeProp(comment) : removeProp(comment),
  );
  return newComments;
}

const removeLocTransform: TransformVisitors = [
  {
    name: 'removeLocTransform',
    enter(path: Path) {
      const {node} = path;
      if (node.loc === undefined) {
        return node;
      } else {
        const newNode: JSNodeBase = {
          ...removeProp(node),
          leadingComments: removeComments(node.leadingComments),
          trailingComments: removeComments(node.trailingComments),
          innerComments: removeComments(node.innerComments),
        };

        // Also remove any `undefined` properties

        // rome-suppress-next-line lint/noExplicitAny
        const escaped: any = newNode;
        for (const key in newNode) {
          if (escaped[key] === undefined) {
            delete escaped[key];
          }
        }

        return (newNode as AnyNode);
      }
    },
  },
];

export default function removeLoc(ast: AnyNode) {
  const context = new Context({
    ast: MOCK_PROGRAM,
    project: {
      folder: undefined,
      config: DEFAULT_PROJECT_CONFIG,
    },
  });
  return context.reduce(ast, removeLocTransform);
}
