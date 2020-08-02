/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticAdvice, DiagnosticDescription} from "../types";
import {flags} from "./flags";
import {parserCore} from "./parsers/parserCore";
import {regexp} from "./parsers/regexp";
import {json} from "./json";
import {semver} from "./semver";
import {v8} from "./v8";
import {lintCommand} from "./commands/lintCommand";
import {projectManager} from "./projectManager";
import {compiler} from "./compiler";
import {stringEscape} from "./stringEscape";
import {analyzeDependencies} from "./analyzeDependencies";
import {stringMarkup} from "./parsers/stringMarkup";
import {pathMatch} from "./parsers/pathMatch";
import {tests} from "./tests";
import {suppressions} from "./suppressions";
import {snapshots} from "./snapshots";
import {bundler} from "./bundler";
import {resolver} from "./resolver";
import {commitParser} from "./commitParser";
import {spdx} from "./parsers/spdx";
import {jsParser} from "./parsers/jsParser";
import {cssParser} from "./parsers/cssParser";
import {typeCheck} from "./typeCheck";
import {consume} from "./consume";
import {manifest} from "./manifest";
import {projectConfig} from "./projectConfig";
import {lint} from "./lint";
import {userConfig} from "./userConfig";
import {htmlParser} from "./parsers/htmlParser";
import {recoveryStore} from "./commands/recoveryStore";
import {markdownParser} from "./parsers/markdownParser";
import {initCommand} from "./commands/initCommand";
import {
	StaticMarkup,
	StaticMarkups,
	concatMarkup,
	markup,
} from "@internal/markup";

export function join(conjunction: string, items: StaticMarkups): StaticMarkup {
	if (items.length === 0) {
		return markup``;
	} else if (items.length === 1) {
		return items[0];
	} else {
		const popped = items.pop()!;
		return concatMarkup(
			[...items, markup`${conjunction} ${popped}`],
			markup`, `,
		);
	}
}

export function andJoin(items: StaticMarkups): StaticMarkup {
	return join("and", items);
}

export function orJoin(items: StaticMarkups): StaticMarkup {
	return join("or", items);
}

export function addEmphasis(items: StaticMarkups): StaticMarkups {
	return items.map((item) => markup`<emphasis>${item}</emphasis>`);
}

// rome-ignore lint/ts/noExplicitAny
type InputMessagesFactory = (
	...params: Array<any>
) => Partial<DiagnosticDescription>;

export type InputMessagesCategory = {
	[key: string]: Partial<DiagnosticDescription> | InputMessagesFactory;
};

type OuputMessagesFactoryReturn<Ret extends Partial<DiagnosticDescription>> = Omit<
	Ret,
	"message" | "advice"
> & {
	advice: DiagnosticAdvice;
	message: StaticMarkup;
};

type OutputMessagesFactory<Func extends InputMessagesFactory> = (
	...params: Parameters<Func>
) => OuputMessagesFactoryReturn<ReturnType<Func>>;

type OutputMessagesValue<Value> = Value extends StaticMarkup
	? {
			message: StaticMarkup;
			advice: DiagnosticAdvice;
		}
	: Value extends Partial<DiagnosticDescription>
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
	// rome-ignore lint/ts/noExplicitAny
	const category: OutputMessagesCategory<any> = {};

	for (const key in input) {
		const value = input[key];

		if (typeof value === "function") {
			// rome-ignore lint/ts/noExplicitAny
			const callback: InputMessagesFactory = (value as any);

			// @ts-ignore trust me lol
			category[key] = function(...params) {
				const {message, advice = [], ...ret} = callback(...params);
				return {
					...ret,
					advice,
					message,
				};
			};
		} else {
			// rome-ignore lint/ts/noExplicitAny
			const {message, advice = [], ...obj} = (value as any);
			category[key] = {
				...obj,
				advice,
				message,
			};
		}
	}

	return category;
}

export const descriptions = {
	COMMIT_PARSER: commitParser,
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
	RECOVERY_STORE: recoveryStore,
	INIT_COMMAND: initCommand,
};
