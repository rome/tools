/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0, Number1} from "@internal/ob1";
import { AnyPath } from "@internal/path";

export type Mappings = Mapping[];

export type ParsedMapping = {
	generated: {
		line: Number1;
		column: Number0;
	};
	original: {
		line: Number1;
		column: Number0;
	};
	source: undefined | AnyPath;
	name: undefined | string;
};

export type Mapping = Omit<ParsedMapping, "generated"> & {
	generated: ParsedMapping["generated"] & {
		index: Number0;
	};
};

export type ParsedMappings = Map<string, Mapping | ParsedMapping>;

export type ResolvedLocation = {
	found: boolean;
	source: AnyPath;
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
	file: string;
	names: string[];
	mappings: string;
	sourceRoot: undefined | string;
	sources: string[];
	sourcesContent: string[];
};
