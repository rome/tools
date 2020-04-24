/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstTSAccessibility, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type ClassPropertyMeta = JSNodeBase & {
  type: 'ClassPropertyMeta';
  static?: boolean;
  accessibility?: ConstTSAccessibility;
  optional?: boolean;
  readonly?: boolean;
  abstract?: boolean;
};

export const classPropertyMeta = createBuilder<ClassPropertyMeta>(
  'ClassPropertyMeta',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);
