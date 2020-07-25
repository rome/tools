/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export type ConstJSSourceType = "script" | "module" | "template";

export type ConstJSExportModuleKind = "type" | "value";

export type ConstJSImportModuleKind = "typeof" | ConstJSExportModuleKind;

export type ConstTSModifier =
	| "readonly"
	| "abstract"
	| "static"
	| "public"
	| "private"
	| "protected";

export type ConstTSAccessibility = "public" | "protected" | "private";

export type ConstJSProgramSyntax = "ts" | "jsx";
