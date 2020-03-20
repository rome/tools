/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBase} from '@romejs/parser-core';
import {AnyComment} from './index';

export type JSNodeBase =
  & NodeBase
  & {
    leadingComments?: Array<AnyComment>;
    trailingComments?: Array<AnyComment>;
    innerComments?: Array<AnyComment>;
  };
