/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  ReferenceIdentifier,
  FlowQualifiedTypeIdentifier,
  FlowTypeParameterInstantiation,
} from '../index';
import {createBuilder} from '../utils';

export type FlowGenericTypeAnnotation =
  & JSNodeBase
  & {
    type: 'FlowGenericTypeAnnotation';
    id: ReferenceIdentifier | FlowQualifiedTypeIdentifier;
    typeParameters?: FlowTypeParameterInstantiation;
  };

export const flowGenericTypeAnnotation =
  createBuilder<FlowGenericTypeAnnotation>('FlowGenericTypeAnnotation', {
    bindingKeys: {},
    visitorKeys: {
      id: true,
      typeParameters: true,
    },
  });
