/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number1, Number0} from '@romejs/ob1';

export type Mappings = Array<Mapping>;

export type Mapping = {
  generated: {
    index: Number0;
    line: Number1;
    column: Number0;
  };
  original: undefined | {
    line: Number1;
    column: Number0;
  };
  source: undefined | string;
  name: undefined | string;
};

export type ParsedMapping = {
  generatedLine: Number1;
  generatedColumn: Number0;
  originalLine: Number1;
  originalColumn: Number0;
  source?: number;
  name?: number;
};

export type ParsedMappings = Map<string, ParsedMapping>;

export type ResolvedLocation = {
  source: string;
  line: Number1;
  column: Number0;
  name: undefined | string;
};

export type SourceMapGeneratorOptions = {
  file?: string;
  sourceRoot?: string;
};

export type SourceMap = {
  version: number;
  file: undefined | string;
  names: Array<string>;
  mappings: string;
  sourceRoot: undefined | string;
  sources: Array<string>;
  sourcesContent: Array<string>;
};
