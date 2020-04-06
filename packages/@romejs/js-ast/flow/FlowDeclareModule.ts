/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  BindingIdentifier,
  StringLiteral,
  BlockStatement,
} from '../index';
import {createBuilder} from '../utils';

export type FlowDeclareModule = JSNodeBase & {
  type: 'FlowDeclareModule';
  id: BindingIdentifier | StringLiteral;
  body: BlockStatement;
  kind?: 'commonjs' | 'es';
};

export const flowDeclareModule = createBuilder<FlowDeclareModule>(
  'FlowDeclareModule',
  {
    bindingKeys: {},
    visitorKeys: {
      id: true,
      body: true,
    },
  },
);
