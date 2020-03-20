/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type DebuggerStatement = JSNodeBase & {type: 'DebuggerStatement'};

export const debuggerStatement = createBuilder<DebuggerStatement>(
  'DebuggerStatement',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);
