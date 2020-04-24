/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyExpression,
  AnyObjectPropertyKey,
  AnyPrimaryType,
  ClassPropertyMeta,
  JSNodeBase,
} from '../index';
import {createBuilder} from '../utils';

export type ClassProperty = JSNodeBase & {
  type: 'ClassProperty';
  key: AnyObjectPropertyKey;
  meta: ClassPropertyMeta;
  value?: AnyExpression;
  typeAnnotation?: AnyPrimaryType;
  definite?: boolean;
};

export const classProperty = createBuilder<ClassProperty>('ClassProperty', {
  bindingKeys: {},
  visitorKeys: {
    key: true,
    meta: true,
    value: true,
    typeAnnotation: true,
  },
});
