/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  BlockStatement,
  ClassPropertyMeta,
  FunctionHead,
  JSNodeBase,
} from '../index';
import {createBuilder} from '../utils';
import {AnyObjectPropertyKey} from '../unions';

export type ClassMethod = JSNodeBase & {
  type: 'ClassMethod';
  meta: ClassPropertyMeta;
  key: AnyObjectPropertyKey;
  kind: ClassMethodKind;
  head: FunctionHead;
  body: BlockStatement;
};

export type ClassMethodKind = 'constructor' | 'method' | 'get' | 'set';

export const classMethod = createBuilder<ClassMethod>('ClassMethod', {
  bindingKeys: {},
  visitorKeys: {
    key: true,
    meta: true,
    head: true,
    body: true,
  },
});
