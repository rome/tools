/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  AnyTypeParameter,
  AnyPrimaryType,
  AnyParamBindingPattern,
  AnyFlowPredicate,
  AnyTargetBindingPattern,
  BindingIdentifier,
} from '../index';
import {createQuickBuilder} from '../utils';

export type FunctionHead =
  & JSNodeBase
  & {
    type: 'FunctionHead';
    params: Array<AnyParamBindingPattern>;
    rest?: AnyTargetBindingPattern;
    thisType?: BindingIdentifier;
    hasHoistedVars?: boolean;
    generator?: boolean;
    async?: boolean;
    typeParameters?: AnyTypeParameter;
    returnType?: AnyPrimaryType;
    predicate?: AnyFlowPredicate;
  };

export const functionHead = createQuickBuilder<FunctionHead, 'params'>(
  'FunctionHead',
  'params',
  {
    bindingKeys: {
      params: true,
      rest: true,
    },
    visitorKeys: {
      params: true,
      thisType: true,
      rest: true,
      returnType: true,
      typeParameters: true,
      predicate: true,
    },
  },
);
