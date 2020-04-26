/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  FlowQualifiedTypeIdentifier,
  FlowTypeParameterInstantiation,
  JSNodeBase,
  ReferenceIdentifier,
} from '../index';
import {createBuilder} from '../utils';

export type FlowInterfaceExtends = JSNodeBase & {
  type: 'FlowInterfaceExtends';
  id: ReferenceIdentifier | FlowQualifiedTypeIdentifier;
  typeParameters?: FlowTypeParameterInstantiation;
};

export const flowInterfaceExtends = createBuilder<FlowInterfaceExtends>(
  'FlowInterfaceExtends',
  {
    bindingKeys: {},
    visitorKeys: {
      id: true,
      typeParameters: true,
    },
  },
);
