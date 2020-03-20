/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, Identifier} from '../index';
import {createBuilder} from '../utils';

export type PrivateName =
  & JSNodeBase
  & {
    type: 'PrivateName';
    id: Identifier;
  };

export const privateName = createBuilder<PrivateName>('PrivateName', {
  bindingKeys: {},
  visitorKeys: {
    id: true,
  },
});
