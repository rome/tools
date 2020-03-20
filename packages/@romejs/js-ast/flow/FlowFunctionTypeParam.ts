/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, PatternMeta, Identifier} from '../index';
import {createBuilder} from '../utils';
import '../patterns/PatternMeta';

export type FlowFunctionTypeParam = 
  & JSNodeBase
  & {
    type: 'FlowFunctionTypeParam';
    name?: Identifier;
    meta: PatternMeta;
  };

export const flowFunctionTypeParam = createBuilder<FlowFunctionTypeParam>(
  'FlowFunctionTypeParam',
  {
    bindingKeys: {},
    visitorKeys: {
      name: true,
      meta: true,
    },
  },
);
