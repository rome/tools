/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, ReferenceIdentifier, Identifier} from '../index';
import {createBuilder} from '../utils';

export type FlowQualifiedTypeIdentifier = JSNodeBase & {
  type: 'FlowQualifiedTypeIdentifier';
  id: Identifier;
  qualification: ReferenceIdentifier | FlowQualifiedTypeIdentifier;
};

export const flowQualifiedTypeIdentifier = createBuilder<
  FlowQualifiedTypeIdentifier
>('FlowQualifiedTypeIdentifier', {
  bindingKeys: {},
  visitorKeys: {
    id: true,
    qualification: true,
  },
});
