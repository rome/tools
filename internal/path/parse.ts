import {HOME_PATH} from ".";
import {PathSegments} from "./types";

type ParsedPathAbsoluteType =
	| "windows-drive"
	| "windows-unc"
	| "posix"
	| "url"
	| "uid";

export type ParsedPath = {
	hint: PathTypeHint;
	absoluteType: ParsedPathAbsoluteType;
	absoluteTarget: undefined | string;
	segments: PathSegments;
	explicitRelative: boolean;
	explicitDirectory: boolean;
};

export function splitPathSegments(str: string): PathSegments {
	// Might be better to do a manual loop to detect escaped slashes or some other weirdness
	return str.split(/[\\\/]/g);
}

export type PathTypeHint = "absolute" | "relative" | "url" | "uid" | "any";

export function parsePathSegments(
	segments: PathSegments,
	hint: PathTypeHint,
	overrides: Pick<Partial<ParsedPath>, "explicitRelative" | "explicitDirectory"> = {

	},
): ParsedPath {
	let absoluteType: ParsedPathAbsoluteType = "posix";
	let absoluteTarget: undefined | string;
	let firstSeg = segments[0] as undefined | string;

	// Detect URL
	if (
		firstSeg !== undefined &&
		!isWindowsDrive(firstSeg) &&
		firstSeg[firstSeg.length - 1] === ":" &&
		segments[1] === ""
	) {
		absoluteTarget = firstSeg.slice(0, -1);

		switch (absoluteTarget) {
			case "file":
				// Automatically normalize a file scheme into an absolute path
				return parsePathSegments(
					segments.slice(2).map((segment) => decodeURIComponent(segment)),
					"absolute",
				);

			// Explicit `uid://foo`
			case "uid":
				return {
					hint: "uid",
					absoluteType: "uid",
					absoluteTarget: undefined,
					explicitDirectory: false,
					explicitRelative: false,
					segments,
				};

			default: {
				const absoluteSegments = segments.slice(0, 3);
				return {
					hint: "absolute",
					absoluteType: "url",
					absoluteTarget,
					...normalizeSegments(
						segments,
						absoluteSegments.length,
						absoluteSegments,
					),
				};
			}
		}
	}

	// UIDs do not have any special segment handling
	if (hint === "uid") {
		return {
			hint: "uid",
			absoluteType: "uid",
			absoluteTarget: undefined,
			explicitDirectory: false,
			explicitRelative: false,
			// UID prefix was not already present
			segments: ["uid:", "", ...segments],
		};
	}

	// Explode home directory
	if ((hint === "absolute" || hint === "any") && firstSeg === "~") {
		segments = [...HOME_PATH.getSegments(), ...segments.slice(1)];
		firstSeg = segments[0];
	}

	let segmentOffset = 0;

	// We first extract the "absolute" portion of a path, this includes any Windows drive letters, UNC hostnames etc
	const absoluteSegments: PathSegments = [];
	if (firstSeg === "") {
		// POSIX path
		absoluteSegments.push("");
		absoluteTarget = "posix";
		segmentOffset++;

		// Windows UNC
		if (segments[1] === "" && segments.length >= 3 && segments[2] !== "") {
			const name = segments[2];
			segmentOffset += 2;
			absoluteSegments.push("");
			absoluteSegments.push(name);
			absoluteType = "windows-unc";
			absoluteTarget = `unc:${name}`;
		}
	} else if (firstSeg !== undefined && isWindowsDrive(firstSeg)) {
		const drive = firstSeg.toUpperCase();
		absoluteSegments.push(drive);
		absoluteType = "windows-drive";
		absoluteTarget = `drive:${drive}`;
		segmentOffset++;
	}

	const {
		explicitDirectory,
		explicitRelative,
		segments: pathSegments,
	} = normalizeSegments(segments, segmentOffset, absoluteSegments);

	return {
		explicitDirectory: overrides.explicitDirectory || explicitDirectory,
		explicitRelative: overrides.explicitRelative || explicitRelative,
		segments: pathSegments,
		absoluteType,
		absoluteTarget,
		hint,
	};
}

function normalizeSegments(
	segments: string[],
	offset: number,
	absoluteSegments: string[],
): {
	explicitDirectory: boolean;
	explicitRelative: boolean;
	segments: string[];
} {
	let explicitDirectory = false;
	let explicitRelative = false;

	const relativeSegments: PathSegments = [];
	for (let i = offset; i < segments.length; i++) {
		let seg = segments[i];

		// Ignore dots, we check for explicit relative below
		if (seg === ".") {
			continue;
		}

		// Ignore empty segments
		if (seg === "") {
			continue;
		}

		// Remove the previous segment, as long as it's not also ..
		if (
			seg === ".." &&
			relativeSegments.length > 0 &&
			relativeSegments[relativeSegments.length - 1] !== ".."
		) {
			relativeSegments.pop();
			continue;
		}

		relativeSegments.push(seg);
	}

	const finalSegments = [...absoluteSegments, ...relativeSegments];

	// Retain explicit directory
	if (
		segments[segments.length - 1] === "" &&
		finalSegments[finalSegments.length - 1] !== "" &&
		relativeSegments.length !== 0
	) {
		explicitDirectory = true;
	}

	explicitRelative =
		absoluteSegments.length === 0 &&
		(segments[0] === "." || segments[0] === "..");

	return {
		explicitDirectory,
		explicitRelative,
		segments: finalSegments,
	};
}

function isWindowsDrive(first: string): boolean {
	return first.length === 2 && first[1] === ":" && /[A-Z]/i.test(first[0]);
}
