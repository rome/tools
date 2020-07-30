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
import {JSONPropertyValue} from "@internal/codec-json";
import {DiagnosticCategory} from "./categories";
import {Dict} from "@internal/typescript-helpers";
import {ClientRequestFlags} from "@internal/core";
import {Markup} from "@internal/markup";

export type DiagnosticFilter = {
	category?: DiagnosticCategory;
	message?: Markup;
	filename?: string;
	start?: Position;
	line?: Number1;
};

export type DiagnosticFilters = Array<DiagnosticFilter>;

export type DiagnosticSuppression = {
	filename: string;
	category: string;
	startLine: Number1;
	endLine: Number1;
	commentLocation: SourceLocation;
};

export type DiagnosticSuppressions = Array<DiagnosticSuppression>;

export type DiagnosticFilterWithTest = DiagnosticFilter & {
	test?: (diagnostic: Diagnostic) => boolean;
};

export type DiagnosticLocation = {
	sourceText?: string;
	mtime?: number;
	marker?: Markup;
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
	| "json"
	| "js"
	| "url"
	| "commit"
	| "shell"
	| "css"
	| "html"
	| "md"
	| "unknown";

export type DiagnosticSourceType = "unknown" | ConstJSSourceType;

export type DiagnosticsMeta = {
	identifierName?: string;
};

export type Diagnostic = {
	description: DiagnosticDescription;
	location: DiagnosticLocation;
	unique?: boolean;
	fixable?: boolean;
	label?: Markup;
	origins?: Array<DiagnosticOrigin>;
	dependencies?: Array<{
		filename: string;
		mtime: number;
	}>;
	meta?: DiagnosticsMeta;
};

export type Diagnostics = Array<Diagnostic>;

export type DiagnosticDescription = {
	category: DiagnosticCategory;
	message: Markup;
	advice: DiagnosticAdvice;
};

export type DiagnosticDescriptionOptionalCategory = {
	category?: DiagnosticCategory;
	message: Markup;
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
	title: Markup;
	advice: DiagnosticAdvice;
};

export type DiagnosticAdviceCommand = {
	type: "command";
	command: string;
};

export type DiagnosticAdviceLog = {
	type: "log";
	category: DiagnosticLogCategory;
	text: Markup;
	compact?: boolean;
};

export type DiagnosticAdviceList = {
	type: "list";
	list: Array<Markup>;
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
	instruction: Markup;
	noun: Markup;
	command: string;
	commandFlags?: Dict<boolean | string | Array<string>>;
	requestFlags?: ClientRequestFlags;
	args?: Array<string>;
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
	title?: Markup;
	truncate?: boolean;
	frames: Array<DiagnosticAdviceStackFrame>;
};

export type DiagnosticAdvice = Array<DiagnosticAdviceItem>;

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
