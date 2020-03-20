/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SourceLocation} from '@romejs/parser-core';
import {Program} from '@romejs/js-ast';
import {HydrateData} from './Evaluator';
import {Dict} from '@romejs/typescript-helpers';

export type CheckProvider = {
  libs?: Array<Program>;
  getExportTypes: (origin: string, relative: string) => Promise<
    | undefined
    | ModuleSignature>;
};

export type TypeCheckProvider = CheckProvider;

export type ModuleSignatureType = {
  human?: string;
  origin: undefined | SourceLocation;
  type: string;
  data: HydrateData;
};

export type ModuleSignatureExport =
  | {
    type: 'local';
    name: string;
    value: string;
  }
  | {
    type: 'all';
    source: string;
  };

export type ModuleSignature = {
  filename: string;
  exports: Array<ModuleSignatureExport>;
  types: Dict<ModuleSignatureType>;
};
