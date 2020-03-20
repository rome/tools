/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyFlowPrimary} from '../index';
import {createBuilder} from '../utils';

export type FlowArrayTypeAnnotation =
  & JSNodeBase
  & {
    type: 'FlowArrayTypeAnnotation';
    elementType: AnyFlowPrimary;
  };

export const flowArrayTypeAnnotation = createBuilder<FlowArrayTypeAnnotation>(
  'FlowArrayTypeAnnotation',
  {
    bindingKeys: {},
    visitorKeys: {
      elementType: true,
    },
  },
);
