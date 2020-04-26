/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyTSEntityName,
  JSNodeBase,
  TSTypeParameterInstantiation,
} from '../index';
import {createBuilder} from '../utils';

export type TSExpressionWithTypeArguments = JSNodeBase & {
  type: 'TSExpressionWithTypeArguments';
  expression: AnyTSEntityName;
  typeParameters?: TSTypeParameterInstantiation;
};

export const tsExpressionWithTypeArguments = createBuilder<TSExpressionWithTypeArguments>(
  'TSExpressionWithTypeArguments',
  {
    bindingKeys: {},
    visitorKeys: {
      expression: true,
      typeParameters: true,
    },
  },
);
