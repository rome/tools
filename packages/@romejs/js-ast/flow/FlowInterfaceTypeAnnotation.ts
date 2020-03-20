/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  FlowInterfaceExtends,
  FlowObjectTypeAnnotation,
} from '../index';
import {createBuilder} from '../utils';

export type FlowInterfaceTypeAnnotation =
  & JSNodeBase
  & {
    type: 'FlowInterfaceTypeAnnotation';
    extends: Array<FlowInterfaceExtends>;
    body: FlowObjectTypeAnnotation;
  };

export const flowInterfaceTypeAnnotation = createBuilder<
  FlowInterfaceTypeAnnotation
>('FlowInterfaceTypeAnnotation', {
  bindingKeys: {},
  visitorKeys: {
    body: true,
    extends: true,
  },
});
