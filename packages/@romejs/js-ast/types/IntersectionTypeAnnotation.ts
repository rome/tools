/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyFlowPrimary, AnyTSPrimary, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type IntersectionTypeAnnotation = JSNodeBase & {
  type: 'IntersectionTypeAnnotation';
  types: Array<AnyFlowPrimary | AnyTSPrimary>;
};

export const intersectionTypeAnnotation = createBuilder<IntersectionTypeAnnotation>(
  'IntersectionTypeAnnotation',
  {
    bindingKeys: {},
    visitorKeys: {
      types: true,
    },
  },
);
