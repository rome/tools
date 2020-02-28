/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from './Scope';
import {AnyNode, ConstImportModuleKind} from '@romejs/js-ast';

let id = 0;

type BindingOpts = {
  scope: Scope;
  node: AnyNode;
  name: string;
  kind?: string;
};

export class Binding {
  constructor(opts: BindingOpts, defaultKind: string = 'variable') {
    this.isExported = false;
    this.scope = opts.scope;
    this.name = opts.name;
    this.node = opts.node;
    this.kind = opts.kind === undefined ? defaultKind : opts.kind;
    this.id = id++;
  }

  id: number;
  name: string;
  kind: string;
  scope: Scope;
  node: AnyNode;
  isExported: boolean;

  setExported(isExported: boolean) {
    this.isExported = isExported;
  }
}

export type ImportBindingMeta =
  | {
      type: 'name';
      imported: string;
      source: string;
      kind: ConstImportModuleKind;
    }
  | {
      type: 'namespace';
      source: string;
      kind: ConstImportModuleKind;
    };

export class ConstBinding extends Binding {
  constructor(
    opts: BindingOpts,
    value: undefined | AnyNode,
    defaultKind?: string,
  ) {
    super(opts, defaultKind);
    this.value = value;
  }

  value: undefined | AnyNode;
}

export class LetBinding extends Binding {}

export class VarBinding extends Binding {}

export class ImportBinding extends Binding {
  constructor(opts: BindingOpts, meta: ImportBindingMeta) {
    super(opts, 'import');
    this.meta = meta;
  }

  meta: ImportBindingMeta;
}

export class ArgumentsBinding extends Binding {
  constructor(opts: BindingOpts) {
    super(opts, 'arguments');
  }
}

export class FunctionBinding extends Binding {
  constructor(opts: BindingOpts) {
    super(opts, 'function');
  }
}

export type TypeBindingKind =
  | 'function'
  | 'class'
  | 'interface'
  | 'typealias'
  | 'parameter';

export class TypeBinding extends ConstBinding {
  constructor(
    opts: BindingOpts,
    valueNode: undefined | AnyNode,
    kind: TypeBindingKind,
  ) {
    super(opts, valueNode, 'type');
    this.typeKind = kind;
  }

  typeKind: TypeBindingKind;
}

export class ClassBinding extends Binding {
  constructor(opts: BindingOpts) {
    super(opts, 'class');
  }
}
