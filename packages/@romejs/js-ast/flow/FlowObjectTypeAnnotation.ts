/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  FlowObjectTypeCallProperty,
  FlowObjectTypeIndexer,
  FlowObjectTypeInternalSlot,
  FlowObjectTypeProperty,
  FlowObjectTypeSpreadProperty,
  JSNodeBase,
} from '../index';
import {createBuilder} from '../utils';

export type FlowObjectTypeAnnotation = JSNodeBase & {
  type: 'FlowObjectTypeAnnotation';
  properties: Array<
    | FlowObjectTypeProperty
    | FlowObjectTypeSpreadProperty
    | FlowObjectTypeIndexer
    | FlowObjectTypeCallProperty
    | FlowObjectTypeInternalSlot
  >;
  exact?: boolean;
  inexact?: boolean;
};

export const flowObjectTypeAnnotation = createBuilder<FlowObjectTypeAnnotation>(
  'FlowObjectTypeAnnotation',
  {
    bindingKeys: {},
    visitorKeys: {
      properties: true,
    },
  },
);
