/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ConstExportModuleKind,
  ConstImportModuleKind,
  AnyNode,
  BindingIdentifier,
  FlowTypeParameter,
  ReferenceIdentifier,
} from '@romejs/js-ast';
import {SourceLocation} from '@romejs/parser-core';
import {
  Path,
  Scope,
  ClassBinding,
  FunctionBinding,
  TypeBinding,
} from '@romejs/js-compiler';
import {AnalyzeExportValueType} from '@romejs/core';

export function isOptional(path: Path): boolean {
  for (const {node} of path.ancestryPaths) {
    if (node.type === 'TryStatement') {
      return true;
    }
  }

  return false;
}

export function isTypeKind(kind: undefined | ConstImportModuleKind): boolean {
  return kind === 'type' || kind === 'typeof';
}

export function getImportKind(
  kind: undefined | ConstImportModuleKind,
): ConstImportModuleKind {
  return kind === undefined ? 'value' : kind;
}

export function getExportKind(
  kind: undefined | ConstExportModuleKind,
): ConstExportModuleKind {
  return kind === undefined ? 'value' : kind;
}

export function maybeTypeBinding(
  kind: ConstExportModuleKind,
  scope: Scope,
  id: BindingIdentifier | FlowTypeParameter | ReferenceIdentifier,
): ConstExportModuleKind {
  const binding = scope.getBinding(id.name);
  if (kind === 'value' && binding instanceof TypeBinding) {
    return 'type';
  } else {
    return kind;
  }
}

export function getKindWithSpecifiers(
  rawKind: undefined | ConstImportModuleKind,
  specifierKinds: Array<ConstImportModuleKind>,
): ConstImportModuleKind {
  const kind: ConstImportModuleKind = getImportKind(rawKind);
  if (isTypeKind(kind) || specifierKinds.length === 0) {
    return kind;
  }

  for (const specifierKind of specifierKinds) {
    if (specifierKind === 'value') {
      return 'value';
    }
  }
  return 'type';
}

// We use this to have an easy way to identify the actual runtime type of an import
// This is useful as we needs this as Flow allows you to `import type` classes which
// are considered values
export function getAnalyzeExportValueType(
  scope: Scope,
  node: undefined | AnyNode,
): AnalyzeExportValueType {
  if (node === undefined) {
    return 'other';
  }

  if (node.type === 'Identifier') {
    const binding = scope.getBinding(node.name);

    if (binding instanceof FunctionBinding) {
      return 'function';
    }

    if (binding instanceof ClassBinding) {
      return 'class';
    }

    if (binding instanceof TypeBinding) {
      const {typeKind} = binding;
      switch (typeKind) {
        case 'function':
        case 'class':
          return typeKind;
      }
    }
  }

  if (node.type === 'FunctionDeclaration') {
    return 'function';
  }

  if (node.type === 'ClassDeclaration' || node.type === 'ClassExpression') {
    return 'class';
  }

  return 'other';
}

// Resolve a export declaration to it's binding node if one exists
export function getDeclarationLoc(
  scope: Scope,
  node: AnyNode,
): undefined | SourceLocation {
  if (node.type === 'ReferenceIdentifier') {
    const binding = scope.getBinding(node.name);
    if (binding !== undefined) {
      return binding.node.loc;
    }
  }

  return node.loc;
}
