/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SourceLocation} from "@internal/parser-core";
import {JSRoot} from "@internal/ast";
import {HydrateData} from "./Evaluator";
import {Dict} from "@internal/typescript-helpers";
import {StaticMarkup} from "@internal/markup";

export type CheckProvider = {
	libs?: Array<JSRoot>;
	getExportTypes: (
		origin: string,
		relative: string,
	) => Promise<undefined | ModuleSignature>;
};

export type TypeCheckProvider = CheckProvider;

export type ModuleSignatureType = {
	human?: StaticMarkup;
	origin: undefined | SourceLocation;
	type: string;
	data: HydrateData;
};

export type ModuleSignatureExport =
	| {
			type: "local";
			name: string;
			value: string;
		}
	| {
			type: "all";
			source: string;
		};

export type ModuleSignature = {
	filename: string;
	exports: Array<ModuleSignatureExport>;
	types: Dict<ModuleSignatureType>;
};
