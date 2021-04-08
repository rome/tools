/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConsumeContext, Consumer} from "@internal/consume";
import {ParserOptions, TokenBase} from "@internal/parser-core";
import {DiagnosticLanguage} from "@internal/diagnostics";
import {JSONValue} from "@internal/codec-config/json/types";
import {TOMLValue} from "./toml/types";

export type ConfigParserOptions = Omit<ParserOptions, "retainCarriageReturn"> & {
	consumeDiagnosticCategoryValue?: string;
};

export type ConfigType = "json" | "toml" | "json5";

export type PartialConsumeConfigResult = {
	consumer: Consumer;
	comments: ConfigCommentMap;
};

export type ConsumeConfigResult = PartialConsumeConfigResult & {
	type: ConfigType;
};

export type PathComments = {
	inner: Comments;
	outer: Comments;
};

export type ConfigCommentMap = Map<string, PathComments>;

export type LineComment = {
	type: "LineComment";
	value: string;
};

export type BlockComment = {
	type: "BlockComment";
	value: string;
};

export type Comments = Array<BlockComment | LineComment>;

export type ConfigParserResult = {
	type: ConfigType;
	value: JSONValue | TOMLValue;
	context: Required<ConsumeContext>;
	comments: ConfigCommentMap;
};

export type PartialConfigHandler = {
	type: ConfigType;
	language: DiagnosticLanguage;
	jsonSuperset: boolean;
	extensions: string[];
	parseExtra: (opts: ParserOptions) => ConfigParserResult;
	tokenize: (
		opts: ConfigParserOptions,
	) => (TokenBase & {
		value?: unknown;
	})[];
	stringifyFromConsumer: (opts: PartialConsumeConfigResult) => string;
};

export type ConfigHandler = PartialConfigHandler & {
	consumeValue: (opts: ConfigParserOptions) => Consumer;
	consume: (opts: ConfigParserOptions) => ConsumeConfigResult;
	parse: (opts: ParserOptions) => unknown;
	stringify: (value: unknown, comments?: ConfigCommentMap) => string;
};
