/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// These are copied from packages/@romejs/codec-json/types.ts
export type JSONValue =
	| null
	| string
	| number
	| boolean
	| JSONObject
	| JSONArray;

export type JSONPropertyValue = undefined | void | JSONValue;

export type JSONObject = {
	[x: string]: JSONPropertyValue;
};

export type JSONArray = Array<JSONValue>;
