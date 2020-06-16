/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export type ModuleMap = Array<[number, string]>;

export type Bundle = {
	pre: string;
	post: string;
	modules: ModuleMap;
};

export type DeltaBundle = {
	added: ModuleMap;
	modified: ModuleMap;
	deleted: Array<number>;
};

export type BundleVariant =
	| ({
			base: true;
			revisionId: string;
		} & Bundle)
	| ({
			base: false;
			revisionId: string;
		} & DeltaBundle);

export type BundleMetadata = {
	pre: number;
	post: number;
	modules: Array<[number, number]>;
};

export type FormattedError = {
	type: string;
	message: string;
	errors: Array<{
		description: string;
	}>;
};

export type HmrModule = {
	module: [number, string];
	sourceMappingURL: string;
	sourceURL: string;
};

export type HmrUpdate = {
	added: Array<HmrModule>;
	deleted: Array<number>;
	isInitialUpdate: boolean;
	modified: Array<HmrModule>;
	revisionId: string;
};

export type HmrServerUpdateMessage = {
	type: "update";
	body: HmrUpdate;
};

export type HmrServerErrorMessage = {
	type: "error";
	body: FormattedError;
};

export type HmrClientLogMessage = {
	type: "log";
	level:
		| "trace"
		| "info"
		| "warn"
		| "log"
		| "group"
		| "groupCollapsed"
		| "groupEnd"
		| "debug";
	data: Array<unknown>;
};

export type HmrClientMessage =
	| {
			type: "register-entrypoints";
			entryPoints: Array<string>;
		}
	| HmrClientLogMessage
	| {
			type: "log-opt-in";
		};

export type HmrServerMessage =
	| {
			type: "bundle-registered";
		}
	| {
			type: "update-start";
			body: {
				isInitialUpdate: boolean;
			};
		}
	| {
			type: "update-done";
		}
	| HmrServerUpdateMessage
	| HmrServerErrorMessage;
