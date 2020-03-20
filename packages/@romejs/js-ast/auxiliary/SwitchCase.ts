/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression, AnyStatement} from '../index';
import {createBuilder} from '../utils';

export type SwitchCase =
  & JSNodeBase
  & {
    type: 'SwitchCase';
    test?: AnyExpression;
    consequent: Array<AnyStatement>;
  };

export const switchCase = createBuilder<SwitchCase>('SwitchCase', {
  bindingKeys: {},
  visitorKeys: {
    test: true,
    consequent: true,
  },
});
