/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

function red(str) {
	return `\u001b[31m${str}\u001b[39m`;
}

// Format of node.version is "v12.6.0" so we want to slice off the v
const versionParts = process.version.slice(1).split(".");
const major = Number(versionParts[0]);

// Keep this updated alongside engines in package.json
const EXPECTED_MAJOR = 12;

if (major < EXPECTED_MAJOR) {
	console.error(
		red(`Rome requires Node >=v${EXPECTED_MAJOR} but got ${process.version}`),
	);
	process.exit(1);
}
