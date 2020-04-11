/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createQuickBuilder} from '../utils';

export type AssignmentIdentifier = JSNodeBase & {
  type: 'AssignmentIdentifier';
  name: string;
  definite?: boolean;
};

export const assignmentIdentifier = createQuickBuilder<
  AssignmentIdentifier,
  'name'
>('AssignmentIdentifier', 'name', {
  bindingKeys: {},
  visitorKeys: {},
});
