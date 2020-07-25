import {createDiagnosticsCategory} from "./index";
import {Markup, markup} from "@romefrontend/cli-layout";
import {buildSuggestionAdvice} from "../helpers";
import {SourceLocation} from "@romefrontend/parser-core";

// @romefrontend/js-analysis
export const typeCheck = createDiagnosticsCategory({
	NOT_CALLABLE: {
		category: "typeCheck/uncallable",
		message: markup`This type isn't callable`,
	},
	INCOMPATIBILITY: (upper: Markup, originLoc: undefined | SourceLocation) => ({
		category: "typeCheck/incompatible",
		message: markup`Type incompatibility found`,
		advice: [
			{
				type: "log",
				category: "error",
				text: markup`This type is incompatible with expected type of`,
			},
			originLoc === undefined
				? {
						type: "log",
						category: "info",
						text: upper,
					}
				: {
						type: "frame",
						location: {
							...originLoc,
							marker: upper,
						},
					},
		],
	}),
	UNKNOWN_IMPORT: (
		importedName: string,
		source: string,
		possibleNames: Array<string>,
	) => ({
		category: "typeCheck/unknownImport",
		message: markup`Unknown import '${importedName}' in '${source}'`,
		advice: buildSuggestionAdvice(importedName, possibleNames),
	}),
	UNKNOWN_PROP: (key: string, possibleNames: Array<string>) => ({
		message: markup`Property ${key} not found in`,
		category: "typeCheck/unknownProperty",
		advice: buildSuggestionAdvice(key, possibleNames),
	}),
	UNDECLARED_VARIABLE: (name: string, possibleNames: Array<string>) => ({
		category: "typeCheck/undeclaredVariable",
		message: markup`Undeclared variable ${name}`,
		advice: buildSuggestionAdvice(name, possibleNames),
	}),
	NOT_EXHAUSTIVE: (only: Markup, target: Markup) => ({
		category: "typeCheck/notExhaustive",
		//message += `but allows ${this.extraenous.map(type => this.utils.humanize(type)).join(' | ')}`;
		message: markup`Expected only a ${only} but got ${target}`,
	}),
	MISSING_CONDITION: (missing: Array<Markup>) => ({
		category: "typeCheck/missingCondition",
		message: markup`Missing the conditions ${missing.join(", ")}`,
	}),
});
