/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyFlowPrimary, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type FlowNullableTypeAnnotation = JSNodeBase & {
  type: 'FlowNullableTypeAnnotation';
  typeAnnotation: AnyFlowPrimary;
};

export const flowNullableTypeAnnotation = createBuilder<
  FlowNullableTypeAnnotation
>('FlowNullableTypeAnnotation', {
  bindingKeys: {},
  visitorKeys: {
    typeAnnotation: true,
  },
});
