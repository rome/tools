/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export type ConstSourceType = 'script' | 'module' | 'template';

export type ConstExportModuleKind = 'type' | 'value';

export type ConstImportModuleKind = 'typeof' | ConstExportModuleKind;

export type ConstTSModifier =
  | 'readonly'
  | 'abstract'
  | 'static'
  | 'public'
  | 'private'
  | 'protected';

export type ConstTSAccessibility = 'public' | 'protected' | 'private';

export type ConstProgramSyntax = 'ts' | 'jsx' | 'flow';
