/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  BindingIdentifier,
  FlowDeclaredPredicate,
  FlowInferredPredicate,
  JSNodeBase,
} from '../index';
import {createBuilder} from '../utils';
import {RequiredProps} from '@romejs/typescript-helpers';

export type FlowDeclareFunction = JSNodeBase & {
  type: 'FlowDeclareFunction';
  id: RequiredProps<BindingIdentifier, 'meta'>;
  predicate?: FlowDeclaredPredicate | FlowInferredPredicate;
};

export const flowDeclareFunction = createBuilder<FlowDeclareFunction>(
  'FlowDeclareFunction',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      predicate: true,
    },
  },
);
