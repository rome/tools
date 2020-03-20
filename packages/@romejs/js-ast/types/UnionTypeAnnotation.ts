/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyFlowPrimary, AnyTSPrimary} from '../index';
import {createBuilder} from '../utils';

export type UnionTypeAnnotation = 
  & JSNodeBase
  & {
    type: 'UnionTypeAnnotation';
    types: Array<AnyFlowPrimary | AnyTSPrimary>;
  };

export const unionTypeAnnotation = createBuilder<UnionTypeAnnotation>(
  'UnionTypeAnnotation',
  {
    bindingKeys: {},
    visitorKeys: {
      types: true,
    },
  },
);
