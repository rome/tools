/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, RegExpSubExpression, RegExpAlternation} from '../index';
import {createBuilder} from '../utils';

export type RegExpLiteral = 
  & JSNodeBase
  & {
    type: 'RegExpLiteral';
    expression: RegExpSubExpression | RegExpAlternation;
    global?: boolean;
    multiline?: boolean;
    sticky?: boolean;
    insensitive?: boolean;
    noDotNewline?: boolean;
    unicode?: boolean;
  };

export const regExpLiteral = createBuilder<RegExpLiteral>('RegExpLiteral', {
  bindingKeys: {},
  visitorKeys: {
    expression: true,
  },
});
