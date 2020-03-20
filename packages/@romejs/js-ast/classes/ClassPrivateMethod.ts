/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ClassMethodKind,
  JSNodeBase,
  ClassPropertyMeta,
  FunctionHead,
  BlockStatement,
  PrivateName,
} from '../index';
import {createBuilder} from '../utils';
import {FlowVariance} from '../flow/FlowVariance';

export type ClassPrivateMethod =
  & JSNodeBase
  & {
    type: 'ClassPrivateMethod';
    kind: ClassMethodKind;
    key: PrivateName;
    head: FunctionHead;
    body: BlockStatement;
    meta: ClassPropertyMeta;
    variance?: FlowVariance;
  };

export const classPrivateMethod = createBuilder<ClassPrivateMethod>(
  'ClassPrivateMethod',
  {
    bindingKeys: {},
    visitorKeys: {
      key: true,
      meta: true,
      head: true,
      body: true,
      variance: true,
    },
  },
);
