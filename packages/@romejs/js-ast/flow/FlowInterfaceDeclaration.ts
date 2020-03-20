/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  BindingIdentifier,
  FlowInterfaceExtends,
  FlowTypeParameterDeclaration,
  FlowObjectTypeAnnotation,
} from '../index';
import {createBuilder} from '../utils';

export type FlowInterfaceDeclaration =
  & JSNodeBase
  & {
    type: 'FlowInterfaceDeclaration';
    id: BindingIdentifier;
    typeParameters?: FlowTypeParameterDeclaration;
    extends: Array<FlowInterfaceExtends>;
    implements: Array<FlowInterfaceExtends>;
    mixins: Array<FlowInterfaceExtends>;
    body: FlowObjectTypeAnnotation;
  };

export const flowInterfaceDeclaration = createBuilder<FlowInterfaceDeclaration>(
  'FlowInterfaceDeclaration',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      typeParameters: true,
      extends: true,
      mixins: true,
      implements: true,
      body: true,
    },
  },
);
