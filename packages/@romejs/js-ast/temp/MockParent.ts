/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type MockParent = JSNodeBase & {
  type: 'MockParent';
};

export const mockParent = createBuilder<MockParent>(
  'MockParent',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);

export const MOCK_PARENT: MockParent = {
  type: 'MockParent',
};
