/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function toCamelCase(str: string, forceCapitalize?: boolean): string {
	// Rest of the code expects at least 1 character
	if (str.length === 0) {
		return str;
	}

	// Prepend uppercase letters with a space unless all characters are uppercase
	str = str.toUpperCase() === str ? ` ${str}` : str.replace(/([A-Z+])/g, " $1");

	// We no longer care about the casing
	str = str.toLowerCase();

	// Capitalize all characters after a symbol or space
	str = str.replace(/[_.\- ]+(\w|$)/g, (_, p1) => p1.toUpperCase());

	// Capitalize characters after a number
	str = str.replace(/\d+(\w|$)/g, (m) => m.toUpperCase());

	// Force capitalize if necessary
	if (forceCapitalize) {
		str = str[0].toUpperCase() + str.slice(1);
	}

	return str;
}
