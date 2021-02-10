/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SPDXLicenseDefinition} from "./types";
import data from "./data";

export {
	SPDXLicenseDefinition,
	SPDXLicenseParserExceptions,
	SPDXLicenseParserOptions,
} from "./types";

const idToLicense: Map<string, SPDXLicenseDefinition> = new Map();
const licenseNames: string[] = [];
for (const license of data.licenses) {
	licenseNames.push(license.licenseId);
	idToLicense.set(license.licenseId.toLowerCase(), license);
}

export {licenseNames};

export {ExpressionNode as SPDXExpressionNode} from "./types";

export function getSPDXLicense(
	licenseId: string,
): undefined | SPDXLicenseDefinition {
	return idToLicense.get(licenseId.toLowerCase());
}

export {stringifySPDXLicense} from "./stringify";
export {parseSPDXLicense} from "./parse";
