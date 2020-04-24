/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AnyNode,
  FlowDeclareExportDefault,
  flowDeclareExportDefault,
} from '@romejs/js-ast';

export default function FlowDeclareExportDefault(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowDeclareExportDefault.assert(node);

  flowDeclareExportDefault.assert(node);
  throw new Error('unimplemented');
}
