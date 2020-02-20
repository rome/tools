/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  AnyExpression,
  AnyClassMember,
  AnyTypeParameter,
  AnyTypeArguments,
  FlowClassImplements,
  TSExpressionWithTypeArguments,
} from '../index';
import {createQuickBuilder} from '../utils';

export type ClassHead = JSNodeBase & {
  type: 'ClassHead';
  superClass?: AnyExpression;
  body: Array<AnyClassMember>;
  typeParameters?: AnyTypeParameter;
  superTypeParameters?: AnyTypeArguments;
  implements?:
    | undefined
    | Array<FlowClassImplements | TSExpressionWithTypeArguments>;
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
