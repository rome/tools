/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DiagnosticAdvice,
	DiagnosticBlessedMessage,
	DiagnosticDescription,
} from "../types";
import {flags} from "./flags";
import {parserCore} from "./parserCore";
import {regexp} from "./regexp";
import {json} from "./json";
import {semver} from "./semver";
import {v8} from "./v8";
import {lintCommand} from "./lintCommand";
import {projectManager} from "./projectManager";
import {compiler} from "./compiler";
import {stringEscape} from "./stringEscape";
import {analyzeDependencies} from "./analyzeDependencies";
import {stringMarkup} from "./stringMarkup";
import {pathMatch} from "./pathMatch";
import {tests} from "./tests";
import {suppressions} from "./suppressions";
import {snapshots} from "./snapshots";
import {bundler} from "./bundler";
import {resolver} from "./resolver";
import {spdx} from "./spdx";
import {jsParser} from "./jsParser";
import {cssParser} from "./cssParser";
import {typeCheck} from "./typeCheck";
import {consume} from "./consume";
import {manifest} from "./manifest";
import {projectConfig} from "./projectConfig";
import {lint} from "./lint";
import {userConfig} from "./userConfig";
import {htmlParser} from "./htmlParser";
import {markdownParser} from "@romefrontend/diagnostics/descriptions/markdownParser";
import {markup} from "@romefrontend/cli-layout";

type DiagnosticMetadataString = Omit<Partial<DiagnosticDescription>, "message"> & {
	message: string;
};

// The purpose of this is so that we're explicit whenever we want to create a diagnostic message outside of this file
export function createBlessedDiagnosticMessage(
	value: string,
): DiagnosticBlessedMessage {
	return {
		type: "PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE",
		value,
	};
}

export function join(conjunction: string, items: Array<string>): string {
	if (items.length === 0) {
		return "";
	} else if (items.length === 1) {
		return items[0];
	} else {
		const popped = items.pop()!;
		return [...items, `${conjunction} ${popped}`].join(", ");
	}
}

export function andJoin(items: Array<string>): string {
	return join("and", items);
}

export function orJoin(items: Array<string>): string {
	return join("or", items);
}

export function addEmphasis(items: Array<string>): Array<string> {
	return items.map((item) => markup`<emphasis>${item}</emphasis>`);
}

// rome-ignore lint/js/noExplicitAny
type InputMessagesFactory = (...params: Array<any>) => DiagnosticMetadataString;

export type InputMessagesCategory = {
	[key: string]: string | DiagnosticMetadataString | InputMessagesFactory;
};

type OuputMessagesFactoryReturn<Ret extends DiagnosticMetadataString> = Omit<
	Ret,
	"message" | "advice"
> & {
	advice: DiagnosticAdvice;
	message: DiagnosticBlessedMessage;
};

type OutputMessagesFactory<Func extends InputMessagesFactory> = (
	...params: Parameters<Func>
) => OuputMessagesFactoryReturn<ReturnType<Func>>;

type OutputMessagesValue<Value> = Value extends string
	? {
			message: DiagnosticBlessedMessage;
			advice: DiagnosticAdvice;
		}
	: Value extends DiagnosticMetadataString
		? OuputMessagesFactoryReturn<Value>
		: Value extends InputMessagesFactory
			? OutputMessagesFactory<Value>
			: never;

type OutputMessagesCategory<Input extends InputMessagesCategory> = {
	[Key in keyof Input]: OutputMessagesValue<Input[Key]>
};

export function createDiagnosticsCategory<Input extends InputMessagesCategory>(
	input: Input,
): OutputMessagesCategory<Input> {
	// rome-ignore lint/js/noExplicitAny
	const category: OutputMessagesCategory<any> = {};

	for (const key in input) {
		const value = input[key];

		if (typeof value === "string") {
			category[key] = {
				advice: [],
				message: createBlessedDiagnosticMessage(value),
			};
		} else if (typeof value === "function") {
			// rome-ignore lint/js/noExplicitAny
			const callback: InputMessagesFactory = (value as any);

			category[key] = function(...params) {
				const {message, advice = [], ...ret} = callback(...params);
				return {
					...ret,
					advice,
					message: createBlessedDiagnosticMessage(message),
				};
			};
		} else {
			// rome-ignore lint/js/noExplicitAny
			const {message, advice = [], ...obj} = (value as any);
			category[key] = {
				...obj,
				advice,
				message: createBlessedDiagnosticMessage(message),
			};
		}
	}

	return category;
}

export const descriptions = {
	FLAGS: flags,
	PARSER_CORE: parserCore,
	REGEX_PARSER: regexp,
	JSON: json,
	SEMVER: semver,
	V8: v8,
	LINT_COMMAND: lintCommand,
	LINT: lint,
	PROJECT_MANAGER: projectManager,
	COMPILER: compiler,
	STRING_ESCAPE: stringEscape,
	ANALYZE_DEPENDENCIES: analyzeDependencies,
	STRING_MARKUP: stringMarkup,
	PATH_MATCH: pathMatch,
	TESTS: tests,
	SUPPRESSIONS: suppressions,
	SNAPSHOTS: snapshots,
	BUNDLER: bundler,
	RESOLVER: resolver,
	SPDX: spdx,
	JS_PARSER: jsParser,
	CSS_PARSER: cssParser,
	TYPE_CHECK: typeCheck,
	CONSUME: consume,
	MANIFEST: manifest,
	PROJECT_CONFIG: projectConfig,
	USER_CONFIG: userConfig,
	HTML_PARSER: htmlParser,
	MARKDOWN_PARSER: markdownParser,
};
