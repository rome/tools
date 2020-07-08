/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export type Platform =
	| "ios"
	| "android"
	| "mobile"
	| "electron"
	| "web"
	| "node";

export const PLATFORMS: Array<Platform> = [
	"ios",
	"android",
	"mobile",
	"electron",
	"web",
	"node",
];

export const PLATFORM_ALIASES = {
	ios: ["mobile"],
	android: ["mobile"],
	electron: ["web"],
	mobile: [],
	node: [],
	web: [],
};
