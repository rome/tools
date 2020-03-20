/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyRegExpBodyItem} from '../index';
import {createBuilder} from '../utils';

export type RegExpSubExpression =
  & JSNodeBase
  & {
    type: 'RegExpSubExpression';
    body: Array<AnyRegExpBodyItem>;
  };

export const regExpSubExpression = createBuilder<RegExpSubExpression>(
  'RegExpSubExpression',
  {
    bindingKeys: {},
    visitorKeys: {
      body: true,
    },
  },
);
