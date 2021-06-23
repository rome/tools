/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConsumerOptions} from "./types";
import Consumer from "./Consumer";
import {RequiredProps, mergeObjects} from "@internal/typescript-helpers";
import {DiagnosticCategory} from "@internal/diagnostics";
import {prettyFormatEager} from "@internal/pretty-format";
import {markupToPlainText} from "@internal/cli-layout";
import {joinMarkupLines} from "@internal/markup";
import {UNKNOWN_PATH} from "@internal/path";

const EMPTY_CONSUME_OPTIONS: Omit<ConsumerOptions, "context"> = {
	propertyMetadata: undefined,
	value: undefined,
	handleUnexpectedDiagnostic: undefined,
	onDefinition: undefined,
	path: UNKNOWN_PATH,
	objectPath: [],
	parent: undefined,
};

export function consume(
	opts: RequiredProps<Partial<ConsumerOptions>, "context">,
): Consumer {
	return new Consumer(
		mergeObjects(
			{
				...EMPTY_CONSUME_OPTIONS,
				context: opts.context,
			},
			opts,
		),
	);
}

export function consumeUnknown(
	value: unknown,
	category: DiagnosticCategory,
	categoryValue?: string,
): Consumer {
	return new Consumer({
		...EMPTY_CONSUME_OPTIONS,
		context: {
			category,
			categoryValue,
			getDiagnosticLocation: (keys) => {
				// TODO support `target` param
				const res = markupToPlainText(
					prettyFormatEager(value, {insertLocator: keys}),
				);
				const locator = res.locators.get("default");
				return {
					path: UNKNOWN_PATH,
					start: locator?.start,
					end: locator?.end,
					sourceText: joinMarkupLines(res),
				};
			},
		},
		value,
	});
}

export {Consumer};

export * from "./types";
export * from "./utils";
