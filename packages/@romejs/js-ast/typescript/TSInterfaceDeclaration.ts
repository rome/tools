/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  BindingIdentifier,
  TSInterfaceBody,
  TSTypeParameterDeclaration,
  TSExpressionWithTypeArguments,
} from '../index';
import {createBuilder} from '../utils';

export type TSInterfaceDeclaration =
  & JSNodeBase
  & {
    type: 'TSInterfaceDeclaration';
    id: BindingIdentifier;
    body: TSInterfaceBody;
    typeParameters?: TSTypeParameterDeclaration;
    extends?: Array<TSExpressionWithTypeArguments>;
    declare?: boolean;
  };

export const tsInterfaceDeclaration = createBuilder<TSInterfaceDeclaration>(
  'TSInterfaceDeclaration',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      body: true,
      typeParameters: true,
      extends: true,
    },
  },
);
