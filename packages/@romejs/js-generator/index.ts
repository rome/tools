/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator, {GeneratorOptions} from './Generator';
import {AnyNode, MOCK_PARENT} from '@romejs/js-ast';

export {GeneratorOptions, Generator};

export function generateJS(
  ast: AnyNode,
  opts: GeneratorOptions,
  code: string = '',
): Generator {
  const generator = new Generator(opts, code);
  generator.print(ast, MOCK_PARENT);
  return generator;
}
