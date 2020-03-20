/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  Directive,
  AnyStatement,
  InterpreterDirective,
  ConstSourceType,
  AnyComment,
  ConstProgramSyntax,
} from '../index';
import {PartialDiagnostics} from '@romejs/diagnostics';
import {createBuilder} from '../utils';

export type Program = 
  & JSNodeBase
  & {
    type: 'Program';
    directives: Array<Directive>;
    body: Array<AnyStatement>;
    filename: string;
    interpreter: undefined | InterpreterDirective;
    mtime: undefined | number;
    corrupt: boolean;
    sourceType: ConstSourceType;
    diagnostics: PartialDiagnostics;
    comments: Array<AnyComment>;
    syntax: Array<ConstProgramSyntax>;
    hasHoistedVars: boolean;
  };

export const MOCK_PROGRAM: Program = {
  type: 'Program',
  directives: [],
  body: [],
  filename: 'unknown',
  mtime: undefined,
  interpreter: undefined,
  corrupt: false,
  sourceType: 'module',
  diagnostics: [],
  comments: [],
  syntax: [],
  hasHoistedVars: false,
};

export const program = createBuilder<Program>('Program', {
  bindingKeys: {},
  visitorKeys: {
    interpreter: true,
    directives: true,
    body: true,
    comments: true,
  },
});
