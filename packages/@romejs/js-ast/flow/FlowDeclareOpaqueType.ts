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

export type FlowDeclareOpaqueType =
  & JSNodeBase
  & {
    type: 'FlowDeclareOpaqueType';
    id: BindingIdentifier;
    typeParameters?: FlowTypeParameterDeclaration;
    supertype?: AnyFlowPrimary;
    impltype?: AnyFlowPrimary;
  };

export const flowDeclareOpaqueType = createBuilder<FlowDeclareOpaqueType>(
  'FlowDeclareOpaqueType',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      typeParameters: true,
      supertype: true,
      impltype: true,
    },
  },
);
