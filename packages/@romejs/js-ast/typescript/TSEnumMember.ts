/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, StringLiteral, Identifier, AnyExpression} from '../index';
import {createBuilder} from '../utils';

export type TSEnumMember = 
  & JSNodeBase
  & {
    type: 'TSEnumMember';
    id: StringLiteral | Identifier;
    initializer?: AnyExpression;
  };

export const tsEnumMember = createBuilder<TSEnumMember>('TSEnumMember', {
  bindingKeys: {},
  visitorKeys: {
    id: true,
    initializer: true,
  },
});
