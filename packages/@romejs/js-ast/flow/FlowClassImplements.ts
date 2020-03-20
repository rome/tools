/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  ReferenceIdentifier,
  FlowTypeParameterInstantiation,
} from '../index';
import {createBuilder} from '../utils';

export type FlowClassImplements =
  & JSNodeBase
  & {
    type: 'FlowClassImplements';
    id: ReferenceIdentifier;
    typeParameters?: FlowTypeParameterInstantiation;
  };

export const flowClassImplements = createBuilder<FlowClassImplements>(
  'FlowClassImplements',
  {
    bindingKeys: {},
    visitorKeys: {
      id: true,
      typeParameters: true,
    },
  },
);
