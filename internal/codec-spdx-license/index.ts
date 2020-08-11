/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import data from "./data";

type License = {
	reference: string;
	isDeprecatedLicenseId: boolean;
	isFsfLibre?: boolean;
	detailsUrl: string;
	referenceNumber: string;
	name: string;
	licenseId: string;
	seeAlso: Array<string>;
	isOsiApproved: boolean;
};

const idToLicense: Map<string, License> = new Map();
const licenseNames: Array<string> = [];
for (const license of data.licenses) {
	licenseNames.push(license.licenseId);
	idToLicense.set(license.licenseId.toLowerCase(), license);
}

export {licenseNames};

export {ExpressionNode as SPDXExpressionNode} from "./parse";

export function getSPDXLicense(licenseId: string): undefined | License {
	return idToLicense.get(licenseId.toLowerCase());
}

export {default as parseSPDXLicense} from "./parse";
export {default as stringifySPDXLicense} from "./stringify";
