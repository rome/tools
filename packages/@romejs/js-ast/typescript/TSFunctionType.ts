/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, TSSignatureDeclarationMeta, AnyPrimaryType} from '../index';
import {createBuilder} from '../utils';

export type TSFunctionType =
  & JSNodeBase
  & {
    type: 'TSFunctionType';
    meta: TSSignatureDeclarationMeta;
    typeAnnotation: AnyPrimaryType;
  };

export const tsFunctionType = createBuilder<TSFunctionType>('TSFunctionType', {
  bindingKeys: {},
  visitorKeys: {
    meta: true,
    typeAnnotation: true,
  },
});
