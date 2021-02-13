import {createDiagnosticsCategory} from "./index";
import {DiagnosticLocation} from "../types";
import {markup} from "@internal/markup";
import {buildSuggestionAdvice} from "../helpers";
import {DiagnosticCategory} from "../categories";
import {ResolverQueryResponseNotFound} from "@internal/core/server/fs/Resolver";
import {SourceLocation} from "@internal/parser-core";
import {AnyPath} from "@internal/path";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

// @internal/path-match
export const resolver = createDiagnosticsCategory({
	NOT_FOUND: (
		responseType: ResolverQueryResponseNotFound["type"],
		source: string,
		location: DiagnosticLocation,
	) => {
		let messagePrefix = "";
		let category: DiagnosticCategory = DIAGNOSTIC_CATEGORIES["resolver/notFound"];

		switch (responseType) {
			case "UNSUPPORTED": {
				messagePrefix = "Unsupported";
				category = DIAGNOSTIC_CATEGORIES["resolver/unsupported"];
				break;
			}
			case "MISSING": {
				messagePrefix = "Cannot find";
				break;
			}
			case "FETCH_ERROR": {
				messagePrefix = "Failed to fetch";
				category = DIAGNOSTIC_CATEGORIES["resolver/fetchFailed"];
				break;
			}
		}

		return {
			message: markup`${messagePrefix} <emphasis>${source}</emphasis> from <emphasis>${location.path}</emphasis>`,
			category,
		};
	},
	UNKNOWN_EXPORT: (
		name: string,
		source: AnyPath,
		exportedNames: string[],
		formatExportedName: (
			name: string,
		) => {
			location: undefined | DiagnosticLocation;
			source: undefined | string;
		},
	) => ({
		message: markup`Couldn't find export <emphasis>${name}</emphasis> in <emphasis>${source}</emphasis>`,
		category: DIAGNOSTIC_CATEGORIES["resolver/unknownExport"],
		advice: exportedNames.length === 0
			? [
					{
						type: "log",
						category: "info",
						text: markup`This file doesn't have any exports`,
					},
				]
			: buildSuggestionAdvice(
					name,
					exportedNames,
					{
						formatItem: (name) => {
							const {location, source} = formatExportedName(name);
							let format = markup`${name}`;

							if (location !== undefined) {
								if (location.start === undefined) {
									format = markup`<filelink target="${location.path.join()}">${name}</filelink>`;
								} else {
									format = markup`<filelink target="${location.path.join()}" line="${String(
										location.start.line,
									)}" column="${String(location.start.column)}">${name}</filelink>`;
								}
							}

							if (source !== undefined) {
								format = markup`${format} <dim>(from <filelink target="${source}" />)</dim>`;
							}

							return format;
						},
					},
				),
	}),
	UNKNOWN_EXPORT_POSSIBLE_UNEXPORTED_LOCAL: (
		name: string,
		source: AnyPath,
		location: SourceLocation,
	) => ({
		message: markup`Couldn't find export <emphasis>${name}</emphasis> in <emphasis>${source}</emphasis>`,
		category: DIAGNOSTIC_CATEGORIES["resolver/unknownExport"],
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`However we found a matching local variable in <emphasis>${location.path!}</emphasis>. Did you forget to export it?`,
			},
			{
				type: "frame",
				location,
			},
		],
	}),
});
