/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0, ob1Add, ob1Coerce0, ob1Inc, ob1Number0} from "@internal/ob1";
import {
	DiagnosticDescriptionOptional,
	descriptions,
} from "@internal/diagnostics";
import {ManifestName} from "./types";

type NormalizeNameUnexpected = (
	opts: {
		description: DiagnosticDescriptionOptional;
		start?: Number0;
		end?: Number0;
		at?: "prefix";
	},
) => void;

type ValidateNamePartOptions = {
	name: string;
	isOrg: boolean;
	isOrgPart: boolean;
	offset: Number0;
};

type NormalizeNameOptions = {
	name: string;
	loose: boolean;
	unexpected: NormalizeNameUnexpected;
};

function validateNamePart(
	{loose, unexpected}: NormalizeNameOptions,
	{name, isOrg, isOrgPart, offset}: ValidateNamePartOptions,
) {
	let normalizedName: string = "";

	for (let i = 0; i < name.length; i++) {
		const char = name[i];

		if (isOrg && char === "@" && i === 0) {
			unexpected({
				description: descriptions.MANIFEST.REDUNDANT_ORG_NAME_START,
				start: ob1Add(offset, i),
			});
		} else if (!isOrgPart && char === "/") {
			/*unexpected({
        at: 'prefix',
        message: `cannot contain any slashes`,
        start: add(offset, i),
        advice: [
          {
            type: 'log',
            category: 'info',
            message:
              'Did you forget a <emphasis>@</emphasis> prefix to make this a scope?',
          },
        ],
      });*/
			normalizedName = `@${normalizedName}/`;
		} else if (!loose && char.match(/[A-Z]/)) {
			/*unexpected({
          at: 'prefix',
          message: `cannot contain uppercase letters`,
          start: add(offset, i),
        });*/
			normalizedName += char.toLowerCase();
		} else if (char.match(/[A-Za-z0-9\-_.]/)) {
			normalizedName += char;
		} else {
			unexpected({
				description: descriptions.MANIFEST.INVALID_NAME_CHAR(char),
				start: ob1Add(offset, i),
			});
		}
	}

	return normalizedName;
}

export function manifestNameToString(name: ManifestName): undefined | string {
	const {packageName, org} = name;

	if (org === undefined) {
		return packageName;
	}

	return `@${org}/${packageName}`;
}

export function normalizeName(opts: NormalizeNameOptions): ManifestName {
	const {unexpected} = opts;
	let {name} = opts;

	let org: undefined | string;
	let packageName: undefined | string;

	if (name.length > 214) {
		unexpected({
			at: "prefix",
			description: descriptions.MANIFEST.NAME_EXCEEDS,
		});
		name = name.slice(0, 214);
	}

	if (name[0] === "." || name[0] === "_") {
		unexpected({
			at: "prefix",
			description: descriptions.MANIFEST.INVALID_NAME_START,
			start: ob1Number0,
		});
		name = name.slice(1);
	}

	if (name[0] === "@") {
		// Validate org and package name separately
		const [rawOrg, rawPackageName, ...other] = name.slice(1).split("/");

		// Leading @
		let offset: Number0 = ob1Coerce0(1);

		// Org
		const sanitizedOrg = validateNamePart(
			opts,
			{
				isOrg: true,
				isOrgPart: true,
				name: rawOrg,
				offset,
			},
		);
		offset = ob1Add(offset, rawOrg.length);
		org = sanitizedOrg;

		if (rawPackageName === undefined) {
			unexpected({
				at: "prefix",
				description: descriptions.MANIFEST.ORG_WITH_NO_PACKAGE_NAME,
				start: offset,
			});
		} else {
			// Forward slashSeparator
			offset = ob1Inc(offset);

			// Package name
			const sanitizedPackageName = validateNamePart(
				opts,
				{
					isOrg: false,
					isOrgPart: true,
					name: rawPackageName,
					offset,
				},
			);
			offset = ob1Add(offset, rawPackageName.length);

			// Complain on excess separators
			if (other.length > 0) {
				unexpected({
					at: "prefix",
					description: descriptions.MANIFEST.ORG_TOO_MANY_PARTS,
					start: offset,
				});
			}

			packageName = sanitizedPackageName;
		}
	} else {
		packageName = validateNamePart(
			opts,
			{
				name,
				offset: ob1Number0,
				isOrg: false,
				isOrgPart: false,
			},
		);
	}

	return {org, packageName};
}
