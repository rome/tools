/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  FunctionHead,
  ClassPropertyMeta,
  ClassMethodKind,
} from '../index';
import {createBuilder} from '../utils';

export type TSDeclareMethod = JSNodeBase & {
  type: 'TSDeclareMethod';
  meta: ClassPropertyMeta;
  kind: ClassMethodKind;
  head: FunctionHead;
  body?: void;
};

export const tsDeclareMethod = createBuilder<TSDeclareMethod>(
  'TSDeclareMethod',
  {
    bindingKeys: {},
    visitorKeys: {
      meta: true,
      head: true,
    },
  },
);
