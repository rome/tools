/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyPrimaryType} from '../index';
import {createBuilder} from '../utils';

export type FlowDeclareModuleExports = JSNodeBase & {
  type: 'FlowDeclareModuleExports';
  typeAnnotation: AnyPrimaryType;
};

export const flowDeclareModuleExports = createBuilder<FlowDeclareModuleExports>(
  'FlowDeclareModuleExports',
  {
    bindingKeys: {},
    visitorKeys: {
      typeAnnotation: true,
    },
  },
);
