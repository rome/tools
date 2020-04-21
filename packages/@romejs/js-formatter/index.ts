/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder, {BuilderOptions} from './Builder';
import {AnyNode, MOCK_PARENT} from '@romejs/js-ast';
import Printer from './Printer';

export {BuilderOptions, Builder};

export function formatJS(ast: AnyNode, opts: BuilderOptions): Printer {
  const builder = new Builder(opts, ast.type === 'Program'
    ? ast.comments
    : opts.comments);
  const tokens = builder.tokenize(ast, MOCK_PARENT);
  const printer = new Printer(opts);
  printer.print(tokens);
  return printer;
}
