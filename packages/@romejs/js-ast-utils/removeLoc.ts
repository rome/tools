/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, MOCK_PROGRAM} from '@romejs/js-ast';
import {DEFAULT_PROJECT_CONFIG} from '@romejs/project';
import {Context, Path} from '@romejs/js-compiler';

const removeLocTransform = [
  {
    name: 'removeLocTransform',
    enter(path: Path) {
      const {node} = path;
      if (node.loc === undefined) {
        return node;
      } else {
        const newNode: any = {...node};
        delete newNode.loc;

        // Also remove any `undefined` properties
        for (const key in newNode) {
          if (newNode[key] === undefined) {
            delete newNode[key];
          }
        }

        return newNode;
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
