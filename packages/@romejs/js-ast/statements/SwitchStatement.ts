/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression, SwitchCase} from '../index';
import {createBuilder} from '../utils';

export type SwitchStatement =
  & JSNodeBase
  & {
    type: 'SwitchStatement';
    discriminant: AnyExpression;
    cases: Array<SwitchCase>;
  };

export const switchStatement = createBuilder<SwitchStatement>(
  'SwitchStatement',
  {
    bindingKeys: {},
    visitorKeys: {
      discriminant: true,
      cases: true,
    },
  },
);
