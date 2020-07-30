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

const EMPTY_CONSUME_OPTIONS: Omit<ConsumerOptions, "context"> = {
	propertyMetadata: undefined,
	value: undefined,
	handleUnexpectedDiagnostic: undefined,
	onDefinition: undefined,
	filePath: undefined,
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
): Consumer {
	return new Consumer({
		...EMPTY_CONSUME_OPTIONS,
		context: {
			category,
		},
		value,
	});
}

export {Consumer};

export * from "./types";
