/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from "@internal/parser-core";
import {CompressedDiff} from "@internal/string-diff";
import {ConstJSSourceType} from "@internal/ast";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {DiagnosticCategory} from "./categories";
import {Dict} from "@internal/typescript-helpers";
import {ClientRequestFlags, CommandName} from "@internal/core";
import {StaticMarkup} from "@internal/markup";
import {MixedPathSet, Path} from "@internal/path";
import {RSERValue} from "@internal/binary-transport";

export type DiagnosticCategoryDescription = {
	category: DiagnosticCategory;
	categoryValue?: string;
};

export type DiagnosticEliminationFilter = {
	category?: DiagnosticCategory;
	message?: StaticMarkup;
	path?: Path;
	start?: Position;
	line?: OneIndexed;
};

export type DiagnosticEliminationFilterWithTest = DiagnosticEliminationFilter & {
	test?: (diagnostic: Diagnostic) => boolean;
};

export type DiagnosticSuppression = DiagnosticCategoryDescription & {
	path: Path;
	startLine: OneIndexed;
	endLine: OneIndexed;
	loc: SourceLocation;
};

export type DiagnosticLocation = {
	sourceText?: string;
	integrity?: DiagnosticIntegrity;
	marker?: StaticMarkup;
	language?: DiagnosticLanguage;
	sourceTypeJS?: DiagnosticSourceType;
	path: Path;
	start?: Position;
	end?: Position;
};

export type DiagnosticOrigin = {
	entity: string;
	message?: StaticMarkup;
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
	| "json5"
	| "js"
	| "mjs"
	| "cjs"
	| "jsx"
	| "url"
	| "urlquery"
	| "commit"
	| "shell"
	| "css"
	| "html"
	| "markdown"
	| "text"
	| "ts"
	| "tsx"
	| "yaml"
	| "toml"
	| "browserquery"
	| "unknown";

export type DiagnosticSourceType = "unknown" | ConstJSSourceType;

export type DiagnosticTag = "fixable" | "internal" | "unique" | "fatal";

export type DiagnosticTags = {[key in DiagnosticTag]?: boolean};

export type Diagnostic = {
	description: DiagnosticDescription;
	location: DiagnosticLocation;
	label?: StaticMarkup;
	origins?: DiagnosticOrigin[];
	dependencies?: DiagnosticDependency[];
	tags?: DiagnosticTags;
};

export type DiagnosticDependency = {
	path: Path;
	integrity?: DiagnosticIntegrity;
};

export type DiagnosticIntegrity = {
	hash: string;
};

export type DiagnosticDescription = DiagnosticCategoryDescription & {
	message: StaticMarkup;
	advice: DiagnosticAdvice[];
	verboseAdvice?: DiagnosticAdvice[];
};

export type DiagnosticDescriptionOptional = {
	category?: DiagnosticCategory;
	categoryValue?: string;
	message: StaticMarkup;
	advice?: DiagnosticAdvice[];
};

export type DiagnosticAdvice =
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
	advice: DiagnosticAdvice[];
};

export type DiagnosticAdviceCommand = {
	type: "command";
	command: string;
};

export type DiagnosticAdviceLog = {
	type: "log";
	category: DiagnosticLogCategory;
	text: StaticMarkup;
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
	data: RSERValue;
};

export type DiagnosticAdviceAction = {
	type: "action";
	command: CommandName;
	commandFlags?: Dict<boolean | string | (string[])>;
	requestFlags?: ClientRequestFlags;
	args?: string[];

	// What will running this command do?
	description: StaticMarkup;

	// Marks actions that aren't important and could be hidden behind a submenu, or ranked at the bottom
	secondary?: boolean;

	// Suggestion for keyboard shortcut to use for interactive execution in an editor or CLI
	suggestedKeyboardShortcut?: string;
};

export type DiagnosticAdviceCode = {
	type: "code";
	sourceText: string;
	truncate?: boolean;
	sourceTypeJS?: ConstJSSourceType;
	language: DiagnosticLanguage;
};

export type DiagnosticAdviceFrame = {
	type: "frame";
	location: DiagnosticLocation;
};

type DiagnosticAdviceDiffBase = {
	language: DiagnosticLanguage;
	sourceTypeJS?: ConstJSSourceType;
	legend?: {
		add: string;
		delete: string;
	};
};

export type DiagnosticAdviceDiff = DiagnosticAdviceDiffBase & {
	type: "diff";
	diff: CompressedDiff[];
};

export type DiagnosticAdviceStacktrace = {
	type: "stacktrace";
	title?: StaticMarkup;
	truncate?: boolean;
	importantPaths?: MixedPathSet;
	frames: DiagnosticAdviceStackFrame[];
};

export type DiagnosticAdviceStackFrame = {
	prefix?: string;
	suffix?: string;
	object?: string;
	property?: string;
	path?: Path;
	line?: OneIndexed;
	column?: ZeroIndexed;
	language: undefined | DiagnosticLanguage;
	sourceText?: string;
};
