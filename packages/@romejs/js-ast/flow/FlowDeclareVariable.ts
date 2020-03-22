/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, BindingIdentifier} from '../index';
import {createBuilder} from '../utils';

export type FlowDeclareVariable =
  & JSNodeBase
  & {
    type: 'FlowDeclareVariable';
    id: BindingIdentifier;
  };

export const flowDeclareVariable = createBuilder<FlowDeclareVariable>(
  'FlowDeclareVariable',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
    },
  },
);
