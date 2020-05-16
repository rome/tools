/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyPrimaryType,
  BindingIdentifier,
  JSNodeBase,
  TSTypeParameterDeclaration,
} from '../index';
import {createBuilder} from '../utils';

export type TypeAliasTypeAnnotation = JSNodeBase & {
  type: 'TypeAliasTypeAnnotation';
  id: BindingIdentifier;
  typeParameters?: TSTypeParameterDeclaration;
  right: AnyPrimaryType;
  declare?: boolean | undefined;
};

export const typeAliasTypeAnnotation = createBuilder<TypeAliasTypeAnnotation>(
  'TypeAliasTypeAnnotation',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      id: true,
      typeParameters: true,
      right: true,
    },
  },
);
