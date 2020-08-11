// Rome is rather strict when it comes to accepting input, this includes validating package.json etc
// However we are integrating into a messy ecosystem. This file defines special-cases for how to handle specific packages.
// Adding an exception here should be a last resort. Adding an exception for a package should satisfy one the following criteria:
// - Hasn't been updated in years
// - Maintainer has abandoned it
// - Bumping the version and propagating it would be extremely difficult or take a long time
import {Consumer} from "@internal/consume";
import {SemverVersionNode, satisfiesSemver} from "@internal/codec-semver";
import {ManifestName} from "@internal/codec-js-manifest/types";
import {manifestNameToString} from "@internal/codec-js-manifest/name";

export const PACKAGE_LICENSE_ALIASES: Map<
	string,
	{
		range: string;
		badLicense: string;
		goodLicense: string;
	}
> = new Map();

// License isn't specific enough. https://github.com/dcporter/didyoumean.js/blob/master/LICENSE
PACKAGE_LICENSE_ALIASES.set(
	"didyoumean",
	{
		range: "^1.0.0",
		badLicense: "Apache",
		goodLicense: "Apache-2.0",
	},
);

export function normalizeCompatManifest(
	consumer: Consumer,
	nameObj: ManifestName,
	version: undefined | SemverVersionNode,
) {
	const name = manifestNameToString(nameObj);
	if (name === undefined) {
		return;
	}

	// Convert bad licenses
	if (version !== undefined && name === "didyoumean") {
		const license = PACKAGE_LICENSE_ALIASES.get(name);
		if (
			license !== undefined &&
			consumer.get("license").asUnknown() === license.badLicense &&
			satisfiesSemver(version, license.range)
		) {
			consumer.set("license", license.goodLicense);
		}
	}
}
