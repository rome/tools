/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  BindingIdentifier,
  BindingObjectPattern,
  BindingArrayPattern,
  AnyTargetBindingPattern,
  AnyPrimaryType,
} from '../index';
import {createBuilder} from '../utils';

export type TSIndexSignature = JSNodeBase & {
  type: 'TSIndexSignature';
  readonly?: boolean;
  parameters: Array<
    BindingIdentifier | BindingObjectPattern | BindingArrayPattern
  >;
  rest: undefined | AnyTargetBindingPattern;
  typeAnnotation: undefined | AnyPrimaryType;
};

export const tsIndexSignature = createBuilder<TSIndexSignature>(
  'TSIndexSignature',
  {
    bindingKeys: {
      parameters: true,
      rest: true,
    },
    visitorKeys: {
      typeAnnotation: true,
      parameters: true,
      rest: true,
    },
  },
);
