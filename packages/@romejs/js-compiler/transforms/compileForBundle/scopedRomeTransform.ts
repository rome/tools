/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {renameBindings} from '@romejs/js-ast-utils';

export default {
  name: 'scopedRome',

  enter(path: Path) {
    const {node, scope} = path;

    if (scope.node === node && scope.hasBinding('Rome')) {
      return renameBindings(
        path,
        new Map([['Rome', scope.generateUid('Rome')]]),
      );
    }

    return node;
  },
};
