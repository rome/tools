/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticCategory,
	DiagnosticLocation,
} from "@romefrontend/diagnostics";
import Consumer from "./Consumer";
import {UnknownFilePath} from "@romefrontend/path";
import {Markup} from "@romefrontend/cli-layout";

export type ConsumeComments = Array<string>;

export type ConsumeKey = number | string;

export type ConsumePath = Array<ConsumeKey>;

export type ConsumeSourceLocationRequestTarget =
	| "all"
	| "key"
	| "value"
	| "inner-value";

export type ConsumeContext = {
	category: DiagnosticCategory;
	normalizeKey?: (key: string) => string;
	getDiagnosticLocation?: (
		keys: ConsumePath,
		target: ConsumeSourceLocationRequestTarget,
	) => DiagnosticLocation;
	getOriginalValue?: (path: ConsumePath) => unknown;
};

export type ConsumePropertyMetadata = {
	description?: Markup;
	inputName?: string;
	getDiagnosticLocation?: (
		target: ConsumeSourceLocationRequestTarget,
	) => DiagnosticLocation;
};

type ConsumePropertyDefinitionBase = {
	objectPath: ConsumePath;
	default: unknown;
	required: boolean;
	metadata: ConsumePropertyMetadata;
};

export type ConsumePropertyPrimitiveDefinition = ConsumePropertyDefinitionBase & {
	type: "boolean" | "bigint" | "date" | "array" | "object";
};

export type ConsumePropertyStringDefinition = ConsumePropertyDefinitionBase & {
	type: "string";
	allowedValues?: Array<string>;
};

export type ConsumePropertyNumberDefinition = ConsumePropertyDefinitionBase & {
	type: "number";
	allowedValues?: Array<number>;
	min?: number;
	max?: number;
};

export type ConsumePropertyDefinition =
	| ConsumePropertyStringDefinition
	| ConsumePropertyPrimitiveDefinition
	| ConsumePropertyNumberDefinition;

export type ConsumerOnDefinition = (
	definition: ConsumePropertyDefinition,
	consumer: Consumer,
) => void;

export type ConsumerHandleUnexpected = (diagnostic: Diagnostic) => void;

export type ConsumerOptions = {
	usedNames?: Iterable<string>;
	handleUnexpectedDiagnostic?: ConsumerHandleUnexpected;
	onDefinition?: ConsumerOnDefinition;
	propertyMetadata?: ConsumePropertyMetadata;
	filePath?: UnknownFilePath;
	objectPath: ConsumePath;
	context: ConsumeContext;
	value: unknown;
	declared?: boolean;
	parent?: Consumer;
	forceDiagnosticTarget?: ConsumeSourceLocationRequestTarget;
};
