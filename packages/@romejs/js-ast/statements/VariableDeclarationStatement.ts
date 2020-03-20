/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createQuickBuilder} from '../utils';
import {VariableDeclaration} from '../auxiliary/VariableDeclaration';

export type VariableDeclarationStatement = 
  & JSNodeBase
  & {
    type: 'VariableDeclarationStatement';
    declaration: VariableDeclaration;
    declare?: boolean;
  };

export const variableDeclarationStatement = createQuickBuilder<
  VariableDeclarationStatement,
  'declaration'
>('VariableDeclarationStatement', 'declaration', {
  bindingKeys: {
    declaration: true,
  },
  visitorKeys: {
    declaration: true,
  },
});
