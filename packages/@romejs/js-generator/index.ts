/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator, {GeneratorOptions} from './Generator';
import {AnyNode} from '@romejs/js-ast';
import {SourceMap, Mappings} from '@romejs/codec-source-map';

export {GeneratorOptions, Generator};

class GeneratorPublic {
  constructor(ast: AnyNode, opts: GeneratorOptions, code: string = '') {
    const generator = new Generator(opts, code);
    generator.print(ast);
    this.generator = generator;
  }

  generator: Generator;

  getCode(): string {
    return this.generator.buf.getCode();
  }

  getSourceMap(): SourceMap {
    return this.generator.buf.getSourceMap();
  }

  getMappings(): Mappings {
    return this.generator.buf.getMappings();
  }
}

export function generateJS(
  ast: AnyNode,
  opts: GeneratorOptions,
  code: string = '',
): GeneratorPublic {
  return new GeneratorPublic(ast, opts, code);
}
