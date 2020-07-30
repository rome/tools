import {createDiagnosticsCategory} from "./index";
import {DiagnosticLocation} from "../types";
import {markup} from "@internal/markup";
import {buildSuggestionAdvice} from "../helpers";
import {DiagnosticCategory} from "../categories";
import {ResolverQueryResponseNotFound} from "@internal/core/server/fs/Resolver";
import {SourceLocation} from "@internal/parser-core";

// @internal/path-match
export const resolver = createDiagnosticsCategory({
	NOT_FOUND: (
		responseType: ResolverQueryResponseNotFound["type"],
		source: string,
		location: DiagnosticLocation,
	) => {
		let messagePrefix = "";
		let category: DiagnosticCategory = "resolver/notFound";

		switch (responseType) {
			case "UNSUPPORTED": {
				messagePrefix = "Unsupported";
				category = "resolver/unsupported";
				break;
			}
			case "MISSING": {
				messagePrefix = "Cannot find";
				break;
			}
			case "FETCH_ERROR": {
				messagePrefix = "Failed to fetch";
				category = "resolver/fetchFailed";
				break;
			}
		}

		return {
			message: markup`${messagePrefix} <emphasis>${source}</emphasis> from <filelink emphasis target="${location.filename!}" />`,
			category,
		};
	},
	UNKNOWN_EXPORT: (
		name: string,
		source: string,
		exportedNames: Array<string>,
		formatExportedName: (
			name: string,
		) => {
			location: undefined | DiagnosticLocation;
			source: undefined | string;
		},
	) => ({
		message: markup`Couldn't find export <emphasis>${name}</emphasis> in <filelink emphasis target="${source}" />`,
		category: "resolver/unknownExport",
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
									format = markup`<filelink target="${location.filename!}">${name}</filelink>`;
								} else {
									format = markup`<filelink target="${location.filename!}" line="${String(
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
		source: string,
		location: SourceLocation,
	) => ({
		message: markup`Couldn't find export <emphasis>${name}</emphasis> in <filelink emphasis target="${source}" />`,
		category: "resolver/unknownExport",
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`However we found a matching local variable in <filelink emphasis target="${location.filename!}" />. Did you forget to export it?`,
			},
			{
				type: "frame",
				location,
			},
		],
	}),
});
