/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSConditionalType = JSNodeBase & {
  type: 'TSConditionalType';
  checkType: AnyTSPrimary;
  extendsType: AnyTSPrimary;
  trueType: AnyTSPrimary;
  falseType: AnyTSPrimary;
};

export const tsConditionalType = createBuilder<TSConditionalType>(
  'TSConditionalType',
  {
    bindingKeys: {},
    visitorKeys: {
      checkType: true,
      extendsType: true,
      trueType: true,
      falseType: true,
    },
  },
);
