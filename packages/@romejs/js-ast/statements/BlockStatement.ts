/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyStatement, Directive} from '../index';
import {createBuilder} from '../utils';

export type BlockStatement = JSNodeBase & {
  type: 'BlockStatement';
  body: Array<AnyStatement>;
  directives?: Array<Directive>;
};

export const blockStatement = createBuilder<BlockStatement>('BlockStatement', {
  bindingKeys: {},
  visitorKeys: {
    body: true,
    directives: true,
  },
});
