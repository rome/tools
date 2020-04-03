/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, ClassHead, BindingIdentifier} from '../index';
import {createBuilder} from '../utils';

export type ClassExpression = JSNodeBase & {
  type: 'ClassExpression';
  id?: BindingIdentifier;
  meta: ClassHead;
};

export const classExpression = createBuilder<ClassExpression>(
  'ClassExpression',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      meta: true,
    },
  },
);
