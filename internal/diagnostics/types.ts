/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from "@internal/parser-core";
import {Diffs} from "@internal/string-diff";
import {ConstJSSourceType} from "@internal/ast";
import {Number0, Number1} from "@internal/ob1";
import {JSONPropertyValue} from "@internal/codec-config";
import {DiagnosticCategory} from "./categories";
import {Dict} from "@internal/typescript-helpers";
import {ClientRequestFlags, CommandName} from "@internal/core";
import {StaticMarkup} from "@internal/markup";

export type DiagnosticFilter = {
	category?: DiagnosticCategory;
	message?: StaticMarkup;
	filename?: string;
	start?: Position;
	line?: Number1;
};

export type DiagnosticSuppression = {
	filename: string;
	category: DiagnosticCategory;
	categoryValue: undefined | string;
	startLine: Number1;
	endLine: Number1;
	loc: SourceLocation;
};

export type DiagnosticSuppressions = DiagnosticSuppression[];

export type DiagnosticFilterWithTest = DiagnosticFilter & {
	test?: (diagnostic: Diagnostic) => boolean;
};

export type DiagnosticLocation = {
	sourceText?: string;
	integrity?: DiagnosticIntegrity;
	marker?: StaticMarkup;
	language?: DiagnosticLanguage;
	sourceTypeJS?: DiagnosticSourceType;
	filename?: string;
	start?: Position;
	end?: Position;
};

export type DiagnosticOrigin = {
	category: string;
	message?: string;
};

export type DiagnosticLogCategory = "none" | "info" | "warn" | "error";

export type DiagnosticLanguage =
	| "spdxlicense"
	| "romemarkup"
	| "semver"
	| "regex"
	| "path"
	| "binary"
	| "rser"
	| "json"
	| "rjson"
	| "json5"
	| "js"
	| "url"
	| "urlquery"
	| "commit"
	| "shell"
	| "css"
	| "html"
	| "markdown"
	| "text"
	| "yaml"
	| "toml"
	| "unknown";

export type DiagnosticSourceType = "unknown" | ConstJSSourceType;

export type DiagnosticTag = "fixable" | "internal" | "unique" | "fatal";

export type DiagnosticTags = {[key in DiagnosticTag]?: boolean};

export type Diagnostic = {
	description: DiagnosticDescription;
	location: DiagnosticLocation;
	label?: StaticMarkup;
	origins?: DiagnosticOrigin[];
	dependencies?: DiagnosticDependencies;
	tags?: DiagnosticTags;
};

export type Diagnostics = Diagnostic[];

export type DiagnosticDependency = {
	filename: string;
	integrity?: DiagnosticIntegrity;
};

export type DiagnosticDependencies = DiagnosticDependency[];

export type DiagnosticIntegrity = {
	hash: string;
};

export type DiagnosticDescription = {
	category: DiagnosticCategory;
	categoryValue?: string;
	message: StaticMarkup;
	advice: DiagnosticAdvice;
};

export type DiagnosticDescriptionOptional = {
	category?: DiagnosticCategory;
	categoryValue?: string;
	message: StaticMarkup;
	advice?: DiagnosticAdvice;
};

export type DiagnosticAdviceItem =
	| DiagnosticAdviceLog
	| DiagnosticAdviceList
	| DiagnosticAdviceInspect
	| DiagnosticAdviceCode
	| DiagnosticAdviceFrame
	| DiagnosticAdviceDiff
	| DiagnosticAdviceStacktrace
	| DiagnosticAdviceCommand
	| DiagnosticAdviceAction
	| DiagnosticAdviceGroup;

export type DiagnosticAdviceGroup = {
	type: "group";
	title: StaticMarkup;
	advice: DiagnosticAdvice;
};

export type DiagnosticAdviceCommand = {
	type: "command";
	command: string;
};

export type DiagnosticAdviceLog = {
	type: "log";
	category: DiagnosticLogCategory;
	text: StaticMarkup;
	compact?: boolean;
};

export type DiagnosticAdviceList = {
	type: "list";
	list: StaticMarkup[];
	truncate?: boolean;
	reverse?: boolean;
	ordered?: boolean;
};

export type DiagnosticAdviceInspect = {
	type: "inspect";
	data: JSONPropertyValue;
};

export type DiagnosticAdviceAction = {
	type: "action";
	hidden?: boolean;
	extra?: boolean;
	shortcut?: string;
	instruction: StaticMarkup;
	noun: StaticMarkup;
	command: CommandName;
	commandFlags?: Dict<boolean | string | (string[])>;
	requestFlags?: ClientRequestFlags;
	args?: string[];
};

export type DiagnosticAdviceCode = {
	type: "code";
	sourceText: string;
	sourceTypeJS?: ConstJSSourceType;
	language: DiagnosticLanguage;
};

export type DiagnosticAdviceFrame = {
	type: "frame";
	location: DiagnosticLocation;
};

export type DiagnosticAdviceDiff = {
	type: "diff";
	diff: Diffs;
	language: DiagnosticLanguage;
	sourceTypeJS?: ConstJSSourceType;
	legend?: {
		add: string;
		delete: string;
	};
};

export type DiagnosticAdviceStacktrace = {
	type: "stacktrace";
	title?: StaticMarkup;
	truncate?: boolean;
	importantFilenames?: string[];
	frames: DiagnosticAdviceStackFrame[];
};

export type DiagnosticAdvice = DiagnosticAdviceItem[];

export type DiagnosticAdviceStackFrame = {
	prefix?: string;
	suffix?: string;
	object?: string;
	property?: string;
	filename?: string;
	line?: Number1;
	column?: Number0;
	language: undefined | DiagnosticLanguage;
	sourceText?: string;
};
