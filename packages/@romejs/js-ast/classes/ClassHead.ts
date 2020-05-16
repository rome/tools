/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyClassMember,
  AnyExpression,
  JSNodeBase,
  TSExpressionWithTypeArguments,
  TSTypeParameterDeclaration,
  TSTypeParameterInstantiation,
} from '../index';
import {createQuickBuilder} from '../utils';

export type ClassHead = JSNodeBase & {
  type: 'ClassHead';
  superClass?: AnyExpression;
  body: Array<AnyClassMember>;
  typeParameters?: TSTypeParameterDeclaration;
  superTypeParameters?: TSTypeParameterInstantiation;
  implements?: undefined | Array<TSExpressionWithTypeArguments>;
};

export const classHead = createQuickBuilder<ClassHead, 'body'>(
  'ClassHead',
  'body',
  {
    bindingKeys: {},
    visitorKeys: {
      superClass: true,
      body: true,
      typeParameters: true,
      superTypeParameters: true,
      implements: true,
    },
  },
);
