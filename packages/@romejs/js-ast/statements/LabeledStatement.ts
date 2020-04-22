/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyStatement, Identifier, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type LabeledStatement = JSNodeBase & {
  type: 'LabeledStatement';
  label: Identifier;
  body: AnyStatement;
};

export const labeledStatement = createBuilder<LabeledStatement>(
  'LabeledStatement',
  {
    bindingKeys: {},
    visitorKeys: {
      label: true,
      body: true,
    },
  },
);
