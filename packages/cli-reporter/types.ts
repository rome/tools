/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Event} from "@romefrontend/events";
import {TerminalFeatures} from "@romefrontend/cli-environment";
import {AnyMarkup, Markup} from "@romefrontend/cli-layout";

export type SelectOption = {
	label: Markup;
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
	defaults?: Array<SelectOptionsKeys<Options>>;
	radio?: boolean;
	yes?: boolean;
};

export type ReporterStreamLineSnapshot = {
	close: () => void;
};

export type ReporterStreamState = {
	lineSnapshots: Map<ReporterStreamLineSnapshot, number>;
	currentLine: number;
	buffer: Array<string>;
	leadingNewline: boolean;
	nextLineInsertLeadingNewline: boolean;
};

export interface ReporterNamespace {
	success: (msg: AnyMarkup) => void;
	info: (msg: AnyMarkup) => void;
	error: (msg: AnyMarkup) => void;
	warn: (msg: AnyMarkup) => void;
	log: (msg: AnyMarkup) => void;
}

export type ReporterConditionalStream = {
	update: () => boolean;
};

export interface ReporterStream {
	features: TerminalFeatures;
	format: "markup" | "ansi" | "html" | "none";
	write: (chunk: string, error: boolean) => void;
	init?: () => void;
	teardown?: () => void;
}

export interface ReporterStreamAttached extends ReporterStream {
	handles: Set<ReporterStreamHandle>;
	state: ReporterStreamState;
	updateFeatures: (features: TerminalFeatures) => void;
}

export interface ReporterStreamHandle {
	stream: ReporterStreamAttached;
	remove: () => void;
}

export type ReporterDerivedStreams = {
	handle: ReporterStreamHandle;
	format: ReporterStream["format"];
	features: TerminalFeatures;
	featuresUpdated: Event<TerminalFeatures, void>;
};

export type ReporterProgressOptions = {
	name?: string;
	title?: Markup;
	initDelay?: number;
	elapsed?: boolean;
	eta?: boolean;
	persistent?: boolean;
};

export type ReporterProgress = {
	render: () => void;
	setCurrent: (current: number) => void;
	setTotal: (total: number, approximate?: boolean) => void;
	setText: (text: Markup) => void;
	pushText: (text: Markup, id?: string) => string;
	popText: (id: string) => void;
	setApproximateETA: (duration: number) => void;
	tick: () => void;
	end: () => void;
	pause: () => void;
	resume: () => void;
};
