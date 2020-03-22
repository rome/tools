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
  FlowInterfaceExtends,
  FlowObjectTypeAnnotation,
} from '../index';
import {createBuilder} from '../utils';

export type FlowDeclareClass =
  & JSNodeBase
  & {
    type: 'FlowDeclareClass';
    id: BindingIdentifier;
    typeParameters?: FlowTypeParameterDeclaration;
    extends: Array<FlowInterfaceExtends>;
    implements: Array<FlowInterfaceExtends>;
    mixins: Array<FlowInterfaceExtends>;
    body: FlowObjectTypeAnnotation;
  };

export const flowDeclareClass = createBuilder<FlowDeclareClass>(
  'FlowDeclareClass',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      typeParameters: true,
      implements: true,
      mixins: true,
      extends: true,
      body: true,
    },
  },
);
