/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder, {BuilderOptions} from './Builder';
import {AnyNode} from '@romejs/js-ast';

export {BuilderOptions, Builder};

export function formatJS(
  ast: AnyNode,
  opts: BuilderOptions,
  code: string = '',
): Builder {
  return new Builder(opts, ast, code);
}
