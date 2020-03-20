/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  FlowTypeParameterDeclaration,
  FlowFunctionTypeParam,
  AnyFlowPrimary,
} from '../index';
import {createBuilder} from '../utils';

export type FlowFunctionTypeAnnotation = 
  & JSNodeBase
  & {
    type: 'FlowFunctionTypeAnnotation';
    typeParameters?: FlowTypeParameterDeclaration;
    params: Array<FlowFunctionTypeParam>;
    rest?: FlowFunctionTypeParam;
    returnType?: AnyFlowPrimary;
  };

export const flowFunctionTypeAnnotation = createBuilder<
  FlowFunctionTypeAnnotation
>('FlowFunctionTypeAnnotation', {
  bindingKeys: {},
  visitorKeys: {
    typeParameters: true,
    params: true,
    rest: true,
    returnType: true,
  },
});
