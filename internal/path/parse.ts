import {HOME_PATH} from ".";
import {PathSegments} from "./types";

interface ParsedPathBase {
	relativeSegments: PathSegments;
	explicitDirectory: boolean;
}

export interface ParsedPathWindowsDrive extends ParsedPathBase {
	type: "windows-drive";
	letter: string;
}

export interface ParsedPathWindowsUNC extends ParsedPathBase {
	type: "windows-unc";
	servername: string;
}

export interface ParsedPathUnix extends ParsedPathBase {
	type: "unix";
}

export interface ParsedPathRelative extends ParsedPathBase {
	type: "relative";
	explicitRelative: boolean;
}

export interface ParsedPathURL extends ParsedPathBase {
	type: "url";
	protocol: string;
	hostname: string;
	port: undefined | number;
	username: undefined | string;
	password: undefined | string;
	search: Map<string, string[]>;
	hash: undefined | string;
}

export interface ParsedPathUID extends ParsedPathBase {
	type: "uid";
}

export type AnyParsedPathAbsolute = ParsedPathWindowsDrive | ParsedPathWindowsUNC | ParsedPathUnix;

export type AnyParsedPath = AnyParsedPathAbsolute | ParsedPathRelative | ParsedPathURL | ParsedPathUID;

export function splitPathSegments(str: string): PathSegments {
	// Might be better to do a manual loop to detect escaped slashes or some other weirdness
	return str.split(/[\\\/]/g);
}

export type PathTypeHint = "absolute" | "relative" | "url" | "uid" | "any";

export type ParsePathSegmentsOverrides = {
	explicitDirectory?: boolean;
};

export function parseRelativePathSegments(
	segments: PathSegments,
	overrides?: ParsePathSegmentsOverrides,
): ParsedPathRelative {
	return {
		type: "relative",
		explicitRelative: (segments[0] === "." || segments[0] === ".."),
		...normalizeSegments(segments, overrides),
	};
}

// Followed is some gnarly regex, be warned!
export function parseURLPathSegments(
	segments: PathSegments,
	overrides?: ParsePathSegmentsOverrides,
): ParsedPathURL {
	const protocol = segments[0];
	let rawHostname: ParsedPathURL["hostname"] = segments[2];
	const relativeSegments = segments.slice(2);

	// Extract username and password
	let username: ParsedPathURL["username"];
	let password: ParsedPathURL["password"];
	const hostCredentialsMatch = rawHostname.match(/^(.*?)@(.*?)$/);
	if (hostCredentialsMatch != null) {
		// If there are multiple @ signs then everything after the last one is the hostname
		rawHostname = hostCredentialsMatch[1];

		// If there are multiple : signs then everything after the first one is the password
		const credentialsMatch = hostCredentialsMatch[2].match(/^([^:]+):(.*?)$/g);
		if (credentialsMatch == null) {
			username = decodeURIComponent(hostCredentialsMatch[2]);
		} else {
			username = decodeURIComponent(credentialsMatch[1]);
			password = decodeURIComponent(credentialsMatch[2]);
		}
	}

	// Extract port
	let port: ParsedPathURL["port"];
	const hostPortMatch = rawHostname.match(/^(.*?):(\d+)$/);
	if (hostPortMatch != null) {
		const maybePort = Number(hostPortMatch[1]);
		if (Number.isInteger(maybePort)) {
			rawHostname = hostPortMatch[0];
			port = maybePort;
		}
	}

	return {
		type: "url",
		protocol,
		hostname: decodeURIComponent(rawHostname),
		port,
		username,
		password,
		...parseURLPathRelativeSegments(relativeSegments, overrides),
	};
}

export function parseURLPathRelativeSegments(
	segments: PathSegments,
	overrides?: ParsePathSegmentsOverrides,
): ParsedPathBase & Pick<ParsedPathURL, "hash" | "search"> {
	// Extract search and hash
	const search: ParsedPathURL["search"] = new Map();
	let hash: ParsedPathURL["hash"];
	if (segments.length > 0) {
		let lastSegment = segments.pop()!;

		// Extract hash
		const hashMatch = lastSegment.match(/^([^#]+)#(.*?)$/);
		if (hashMatch != null) {
			lastSegment = hashMatch[1];
			hash = decodeURIComponent(hashMatch[2]);
		}

		// Extract search
		const searchMatch = lastSegment.match(/^([^?]+)\?(.*?)$/);
		if (searchMatch != null) {
			lastSegment = searchMatch[1];

			const pairs = searchMatch[2].split("&");
			for (const pair of pairs) {
				if (pair === "") {
					continue;
				}

				const keyMatch = pair.match(/^([^=]+)=(.*?)$/);
				let key;
				let value;
				if (keyMatch == null) {
					key = decodeURIComponent(pair);
					value = "";
				} else {
					key = decodeURIComponent(keyMatch[1]);
					value = decodeURIComponent(keyMatch[2]);
				}

				let values = search.get(key);
				if (values === undefined) {
					values = [];
					search.set(key, values);
				}
				values.push(value);
			}
		}

		segments.push(lastSegment);
	}

	return {
		search,
		hash,
		...normalizeSegments(segments.map((segment) => decodeURIComponent(segment)), overrides),
	};
}

export function parsePathSegments(
	segments: PathSegments,
	hint: PathTypeHint,
	overrides?: ParsePathSegmentsOverrides,
): AnyParsedPath {
	// Detect URL
	let firstSeg = segments[0] as undefined | string;
	if (
		firstSeg !== undefined &&
		!isWindowsDrive(firstSeg) &&
		firstSeg[firstSeg.length - 1] === ":" &&
		segments[1] === ""
	) {
		// Explicit `uid://foo`
		if (firstSeg === "uid:") {
			return {
				type: "uid",
				explicitDirectory: false,
				relativeSegments: segments.slice(2),
			};
		}

		if (firstSeg === "file:") {
			// Automatically normalize a file scheme into an absolute path
			return parsePathSegments(segments.slice(2).map((segment) => decodeURIComponent(segment)), "absolute", overrides);
		}

		return parseURLPathSegments(segments, overrides);
	}

	// UIDs do not have any special segment handling
	if (hint === "uid") {
		return {
			type: "uid",
			explicitDirectory: false,
			relativeSegments: segments,
		};
	}

	// Explode home directory
	if ((hint === "absolute" || hint === "any") && segments[0] === "~") {
		return {
			...HOME_PATH.parsed,
			...normalizeSegments([...HOME_PATH.getSegments(), ...segments.slice(1)], overrides),
		};
	}

	// Detect absolute paths
	if (segments[0] === "") {
		// Windows UNC: \\servername\path
		if (segments[1] === "" && segments.length >= 3 && segments[2] !== "") {
			return {
				type: "windows-unc",
				servername: segments[2],
				...normalizeSegments(segments.slice(3), overrides),
			};
		}

		// POSIX path: /home/sebmck
		return {
			type: "unix",
			...normalizeSegments(segments.slice(1), overrides),
		};
	}
	
	// Windows drive: C:\Users\Sebastian
	if (segments.length > 0 && isWindowsDrive(segments[0])) {
		return {
			type: "windows-drive",
			letter: segments[0][0].toUpperCase(),
			...normalizeSegments(segments.slice(1), overrides),
		};
	}

	return parseRelativePathSegments(segments, overrides);
}

export function normalizeSegments(
	segments: string[],
	overrides?: ParsePathSegmentsOverrides,
): ParsedPathBase {
	let explicitDirectory = false;

	const relativeSegments: PathSegments = [];

	for (const seg of segments) {
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

	// Retain explicit directory
	if (relativeSegments[relativeSegments.length - 1] === "") {
		explicitDirectory = true;
	}

	if (overrides !== undefined && overrides.explicitDirectory) {
		explicitDirectory = true;
	}

	return {
		explicitDirectory,
		relativeSegments,
	};
}

function isWindowsDrive(first: string): boolean {
	return first.length === 2 && first[1] === ":" && /[A-Z]/i.test(first[0]);
}
