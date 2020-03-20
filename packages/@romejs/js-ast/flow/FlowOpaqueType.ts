/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  BindingIdentifier,
  FlowTypeParameterDeclaration,
  AnyFlowPrimary,
} from '../index';
import {createBuilder} from '../utils';

export type FlowOpaqueType = 
  & JSNodeBase
  & {
    type: 'FlowOpaqueType';
    id: BindingIdentifier;
    typeParameters?: FlowTypeParameterDeclaration;
    supertype?: AnyFlowPrimary;
    impltype?: AnyFlowPrimary;
  };

export const flowOpaqueType = createBuilder<FlowOpaqueType>('FlowOpaqueType', {
  bindingKeys: {
    id: true,
  },
  visitorKeys: {
    id: true,
    typeParameters: true,
    impltype: true,
    supertype: true,
  },
});
