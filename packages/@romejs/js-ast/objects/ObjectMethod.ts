/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  StaticPropertyKey,
  ComputedPropertyKey,
  BlockStatement,
  FunctionHead,
} from '../index';
import {createBuilder} from '../utils';

export type ObjectMethodKind = 'get' | 'set' | 'method';

export type ObjectMethod = 
  & JSNodeBase
  & {
    key: ComputedPropertyKey | StaticPropertyKey;
    type: 'ObjectMethod';
    kind: ObjectMethodKind;
    head: FunctionHead;
    body: BlockStatement;
  };

export const objectMethod = createBuilder<ObjectMethod>('ObjectMethod', {
  bindingKeys: {},
  visitorKeys: {
    key: true,
    head: true,
    body: true,
  },
});
