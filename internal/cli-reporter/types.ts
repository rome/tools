/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Event} from "@internal/events";
import {TerminalFeatures} from "@internal/cli-environment";
import {AnyMarkup, AnyMarkups, StaticMarkup} from "@internal/markup";
import {ZeroIndexed} from "@internal/numbers";
import {
	AsyncCallback,
	AsyncVoidCallback,
	VoidCallback,
} from "@internal/typescript-helpers";
import { Resource } from "@internal/resources";

// rome-ignore lint/ts/noExplicitAny: future cleanup
export type WrapperFactory = <T extends (...args: any[]) => any>(
	callback: T,
) => T;

export type SelectOption = {
	label: StaticMarkup;
	shortcut?: string;
};

export type SelectOptionsKeys<Options extends SelectOptions> = Extract<
	keyof Options,
	string
>;

export type SelectOptions = {
	[key: string]: undefined | SelectOption;
};

export type SelectArguments<Options extends SelectOptions> = {
	options: Options;
	defaults?: SelectOptionsKeys<Options>[];
	radio?: boolean;
	yes?: boolean;
};

export type ReporterStepCallback = {
	message: AnyMarkup;
	test?: AsyncCallback<boolean>;
	callback: AsyncVoidCallback;
};

export interface ReporterListOptions {
	prefix?: AnyMarkup;
	reverse?: boolean;
	truncate?: number;
	ordered?: boolean;
	pad?: boolean;
	start?: number;
}

export type ReporterStreamLineSnapshot = {
	close: VoidCallback;
};

export type ReporterStreamState = {
	lineSnapshots: Map<ReporterStreamLineSnapshot, ZeroIndexed>;
	currentLine: ZeroIndexed;
	buffer: string[];
	leadingNewline: boolean;
	nextLineInsertLeadingNewline: boolean;
};

export interface ReporterNamespace {
	success: (msg: AnyMarkup) => void;
	info: (msg: AnyMarkup) => void;
	error: (msg: AnyMarkup) => void;
	warn: (msg: AnyMarkup) => void;
	log: (msg: AnyMarkup) => void;
	list: (items: AnyMarkups, opts?: ReporterListOptions) => void;
	namespace: (...prefixes: AnyMarkup[]) => ReporterNamespace;
}

export type ReporterConditionalStream = {
	enable: () => Promise<void>;
	disable: () => Promise<void>;
	update: () => Promise<boolean>;
};

export type ReporterCaptureStream = {
	read: () => string;
	readAsMarkup: () => StaticMarkup;
	resources: Resource;
};

export interface ReporterStream {
	features: TerminalFeatures;
	format: "markup" | "ansi" | "html" | "none";
	write: (chunk: string, error: boolean) => void;
}

export interface ReporterStreamAttached extends ReporterStream {
	state: ReporterStreamState;
	updateFeatures: (features: TerminalFeatures) => Promise<void>;
	featuresUpdated: Event<TerminalFeatures, void>;
	resources: Resource;
}

export type ReporterDerivedStreams = {
	stream: ReporterStreamAttached;
	format: ReporterStream["format"];
	features: TerminalFeatures;
	featuresUpdated: Event<TerminalFeatures, void>;
};

export type ReporterProgressOptions = {
	name?: string;
	title?: StaticMarkup;
	initDelay?: number;
	elapsed?: boolean;
	eta?: boolean;
	persistent?: boolean;
};

export type ReporterProgress = {
	render: VoidCallback;
	setCurrent: (current: number) => void;
	setTotal: (total: number, approximate?: boolean) => void;
	setText: (text: AnyMarkup) => void;
	pushText: (text: AnyMarkup, id?: string) => string;
	popText: (id: string) => void;
	setApproximateETA: (duration: number) => void;
	tick: VoidCallback;
	end: VoidCallback;
	pause: VoidCallback;
	resume: VoidCallback;
};
