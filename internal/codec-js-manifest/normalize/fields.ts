import {Consumer} from "@internal/consume";
import {descriptions} from "@internal/diagnostics";
import {RelativePathMap, createRelativePath} from "@internal/path";
import {
	Manifest,
	ManifestExportConditions,
	ManifestExportNestedCondition,
	ManifestExportRelativeCondition,
	ManifestExportsField,
	ManifestName,
	ManifestPersonField,
} from "../types";
import {parseGitDependencyPattern} from "./dependencies";
import {normalizeName} from "./name";

export function normalizePersonField(
	consumer: Consumer,
	loose: boolean,
): ManifestPersonField {
	if (typeof consumer.asUnknown() === "string") {
		// Parse the string. Format: name (url) <email>
		const str = consumer.asString();

		const nameMatch = str.match(/^([^(<]+)/);
		let name: string | undefined;
		if (nameMatch) {
			name = nameMatch[0].trim();
		}

		const person: ManifestPersonField = {
			name,
			url: undefined,
			email: undefined,
			twitter: undefined,
			github: undefined,
		};

		const emailMatch = str.match(/<([^>]+)>/);
		if (emailMatch) {
			person.email = emailMatch[1];
		}

		const urlMatch = str.match(/\(([^)]+)\)/);
		if (urlMatch) {
			person.url = urlMatch[1];
		}

		return person;
	} else {
		// Validate as an object
		let url: string | undefined = consumer.get("url").asStringOrVoid();

		// Some packages use "web" or "website" instead of "url"
		if (loose) {
			if (url === undefined) {
				url = consumer.get("web").asStringOrVoid();
			}

			if (url === undefined) {
				url = consumer.get("website").asStringOrVoid();
			}
		}

		let github = consumer.get("github").asStringOrVoid();

		if (loose && github === undefined) {
			// Some rando packages use this
			github =
				consumer.get("githubUsername").asStringOrVoid() ||
				consumer.get("github-username").asStringOrVoid();
		}

		const person: ManifestPersonField = {
			name: consumer.get("name").required(loose ? "" : undefined).asString(),
			email: consumer.get("email").asStringOrVoid(),
			twitter: consumer.get("twitter").asStringOrVoid(),
			github,
			url,
		};
		if (!loose) {
			consumer.enforceUsedProperties();
		}
		return person;
	}
}

export function normalizePeopleField(
	consumer: Consumer,
	loose: boolean,
): undefined | (ManifestPersonField[]) {
	if (!consumer.exists()) {
		return;
	}

	// Some packages have a single maintainer object instead of an array
	if (loose && consumer.isObject()) {
		return [normalizePersonField(consumer, loose)];
	}

	// If it's not an array then just leave it. Some people put a URL here.
	if (loose && !Array.isArray(consumer.asUnknown())) {
		return;
	}

	const people: ManifestPersonField[] = [];

	for (const item of consumer.asIterable()) {
		people.push(normalizePersonField(item, loose));
	}

	return people;
}

export function normalizeRepoField(
	consumer: Consumer,
	loose: boolean,
): Manifest["repository"] {
	if (!consumer.exists()) {
		return;
	}

	if (typeof consumer.asUnknown() === "string") {
		let url = consumer.asString();

		// If this is a hosted git shorthand then explode it
		const parsed = parseGitDependencyPattern(consumer);
		if (parsed?.type === "hosted-git") {
			url = parsed.url;
		}

		return {
			type: "git",
			url,
			directory: undefined,
		};
	} else {
		let url: string;
		let type: string;

		if (loose) {
			// A lot of packages omit the "type"
			type = consumer.get("type").required("git").asString();

			// thanks i hate it
			consumer.markUsedProperty("web");
			consumer.markUsedProperty("git");
			consumer.markUsedProperty("dist");

			// Some gross packages use "repository" instead of "url"
			let looseUrl = consumer.get("url").asStringOrVoid();

			if (looseUrl === undefined) {
				looseUrl = consumer.get("repository").asStringOrVoid();
			}

			if (looseUrl === undefined) {
				consumer.unexpected(descriptions.MANIFEST.MISSING_REPO_URL);
				url = "";
			} else {
				url = looseUrl;
			}
		} else {
			url = consumer.get("url").asString();
			type = consumer.get("type").asString();
		}

		const repo: Manifest["repository"] = {
			type,
			url,
			directory: consumer.get("directory").asStringOrVoid(),
		};
		if (!loose) {
			consumer.enforceUsedProperties();
		}
		return repo;
	}
}

export function normalizeExportsField(
	consumer: Consumer,
): boolean | ManifestExportsField {
	return true;
	const unknown = consumer.asUnknown();

	// "exports": false
	if (typeof unknown === "boolean") {
		return consumer.asBoolean();
	}

	if (!consumer.exists()) {
		return true;
	}

	const exports: ManifestExportsField = new RelativePathMap();

	// "exports": "./index.js"
	if (typeof unknown === "string") {
		exports.set(
			createRelativePath("."),
			new Map([["default", createRelativeExportCondition(consumer)]]),
		);
		return exports;
	}

	let dotConditionCount = 0;

	for (const [relative, value] of consumer.asMap()) {
		if (relative[0] !== ".") {
			if (exports.size > 0) {
				value.unexpected(descriptions.MANIFEST.MIXED_EXPORTS_PATHS);
			}

			dotConditionCount++;
		}

		const conditions = normalizeExportsConditions(value);
		exports.set(value.getKey().asRelativePath(), conditions);
	}

	if (dotConditionCount && dotConditionCount !== exports.size) {
		consumer.unexpected(descriptions.MANIFEST.MIXED_EXPORTS_PATHS);
	}

	return exports;
}

function createRelativeExportCondition(
	value: Consumer,
): ManifestExportRelativeCondition {
	return {
		type: "relative",
		consumer: value,
		relative: value.asExplicitRelativePath(),
	};
}

function normalizeExportsConditions(value: Consumer): ManifestExportConditions {
	const conditions: ManifestExportConditions = new Map();
	const unknown = value.asUnknown();

	if (typeof unknown === "string") {
		conditions.set("default", createRelativeExportCondition(value));
	} else if (Array.isArray(unknown)) {
		// Find the first item that passes validation
		for (const elem of value.asIterable()) {
			const {consumer, diagnostics} = elem.capture();
			const result = normalizeExportsConditions(consumer);
			if (diagnostics.length === 0) {
				return result;
			}
		}
	} else {
		for (const [type, prop] of value.asMap()) {
			if (prop.isObject()) {
				const condition: ManifestExportNestedCondition = {
					type: "nested",
					consumer: prop,
					conditions: new Map(),
				};

				for (const [name, subprop] of prop.asMap()) {
					condition.conditions.set(name, createRelativeExportCondition(subprop));
				}

				conditions.set(type, condition);
			} else {
				conditions.set(type, createRelativeExportCondition(prop));
			}
		}
	}

	return conditions;
}

export function normalizeBugsField(
	consumer: Consumer,
	loose: boolean,
): undefined | Manifest["bugs"] {
	if (!consumer.exists()) {
		return;
	}

	if (typeof consumer.asUnknown() === "string") {
		return {
			email: undefined,
			url: consumer.asString(),
		};
	} else {
		let email = consumer.get("email").asStringOrVoid();

		// Some use a `mail` property
		if (loose && email === undefined) {
			email = consumer.get("mail").asStringOrVoid();
		}

		// TODO remove this
		consumer.markUsedProperty("type");

		const bugs: Manifest["bugs"] = {
			email,
			url: consumer.get("url").asStringOrVoid(),
		};
		if (!loose) {
			consumer.enforceUsedProperties();
		}
		return bugs;
	}
}

export function normalizeNameField(
	consumer: Consumer,
	loose: boolean,
): ManifestName {
	if (!consumer.has("name")) {
		return {
			packageName: undefined,
			org: undefined,
		};
	}

	const prop = consumer.get("name");

	return normalizeName({
		name: prop.asString(),
		loose,
		unexpected: ({description, start, end}) => {
			prop.unexpected(
				description,
				{
					loc: start === undefined
						? undefined
						: prop.getLocationRange(start, end, "inner-value"),
				},
			);
		},
	});
}
