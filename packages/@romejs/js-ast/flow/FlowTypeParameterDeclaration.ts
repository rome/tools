/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, FlowTypeParameter} from '../index';
import {createBuilder} from '../utils';

export type FlowTypeParameterDeclaration = 
  & JSNodeBase
  & {
    type: 'FlowTypeParameterDeclaration';
    params: Array<FlowTypeParameter>;
  };

export const flowTypeParameterDeclaration = createBuilder<
  FlowTypeParameterDeclaration
>('FlowTypeParameterDeclaration', {
  bindingKeys: {},
  visitorKeys: {
    params: true,
  },
});
