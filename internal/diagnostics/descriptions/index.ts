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
import {files} from "./files";
import {compiler} from "./compiler";
import {stringEscape} from "./stringEscape";
import {analyzeDependencies} from "./analyzeDependencies";
import {stringMarkup} from "./parsers/stringMarkup";
import {pathMatch} from "./parsers/pathMatch";
import {tests} from "./tests";
import {integrations} from "./integrations";
import {suppressions} from "./suppressions";
import {snapshots} from "./snapshots";
import {bundler} from "./bundler";
import {resolver} from "./resolver";
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
import {autoConfigCommand} from "./commands/autoConfigCommand";
import {StaticMarkup, joinMarkup, markup} from "@internal/markup";
import {toml} from "./parsers/tomlParser";
import {browserquery} from "./parsers/browserquery";
import {vcs} from "@internal/diagnostics/descriptions/vcs";
import {migrateCommand} from "@internal/diagnostics/descriptions/commands/migrateCommand";

export function join(conjunction: string, items: StaticMarkup[]): StaticMarkup {
	if (items.length === 0) {
		return markup``;
	} else if (items.length === 1) {
		return items[0];
	} else {
		const popped = items.pop()!;
		return joinMarkup([...items, markup`${conjunction} ${popped}`], markup`, `);
	}
}

export function andJoin(items: StaticMarkup[]): StaticMarkup {
	return join("and", items);
}

export function orJoin(items: StaticMarkup[]): StaticMarkup {
	return join("or", items);
}

export function addEmphasis(items: StaticMarkup[]): StaticMarkup[] {
	return items.map((item) => markup`<emphasis>${item}</emphasis>`);
}

// rome-ignore lint/ts/noExplicitAny: future cleanup
type InputMessagesFactory = (...params: any[]) => Partial<DiagnosticDescription>;

export type InputMessagesCategory = {
	[key: string]: Partial<DiagnosticDescription> | InputMessagesFactory;
};

type OuputMessagesFactoryReturn<Ret extends Partial<DiagnosticDescription>> = Omit<
	Ret,
	"message" | "advice"
> & {
	advice: DiagnosticAdvice[];
	message: StaticMarkup;
};

type OutputMessagesFactory<Func extends InputMessagesFactory> = (
	...params: Parameters<Func>
) => OuputMessagesFactoryReturn<ReturnType<Func>>;

type OutputMessagesValue<Value> = Value extends StaticMarkup
	? {
			message: StaticMarkup;
			advice: DiagnosticAdvice[];
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
	// rome-ignore lint/ts/noExplicitAny: future cleanup
	const category: OutputMessagesCategory<any> = {};

	for (const key in input) {
		const value = input[key];

		if (typeof value === "function") {
			// rome-ignore lint/ts/noExplicitAny: future cleanup
			const callback: InputMessagesFactory = value as any;

			// @ts-expect-error trust me lol
			category[key] = function(...params) {
				const {message, advice = [], ...ret} = callback(...params);
				return {
					...ret,
					advice,
					message,
				};
			};
		} else {
			// rome-ignore lint/ts/noExplicitAny: future cleanup
			const {message, advice = [], ...obj} = value as any;
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
	INTEGRATIONS: integrations,
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
	FILES: files,
	USER_CONFIG: userConfig,
	HTML_PARSER: htmlParser,
	MARKDOWN_PARSER: markdownParser,
	RECOVERY_STORE: recoveryStore,
	INIT_COMMAND: autoConfigCommand,
	MIGRATE_COMMAND: migrateCommand,
	TOML: toml,
	VCS: vcs,
	BROWSERQUERY: browserquery,
};
