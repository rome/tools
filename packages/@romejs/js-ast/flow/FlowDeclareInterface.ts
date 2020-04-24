/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  BindingIdentifier,
  FlowInterfaceExtends,
  FlowObjectTypeAnnotation,
  FlowTypeParameterDeclaration,
  JSNodeBase,
} from '../index';
import {createBuilder} from '../utils';

export type FlowDeclareInterface = JSNodeBase & {
  type: 'FlowDeclareInterface';
  id: BindingIdentifier;
  typeParameters?: FlowTypeParameterDeclaration;
  extends: Array<FlowInterfaceExtends>;
  implements: Array<FlowInterfaceExtends>;
  mixins: Array<FlowInterfaceExtends>;
  body: FlowObjectTypeAnnotation;
};

export const flowDeclareInterface = createBuilder<FlowDeclareInterface>(
  'FlowDeclareInterface',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      typeParameters: true,
      extends: true,
      implements: true,
      mixins: true,
      body: true,
    },
  },
);
