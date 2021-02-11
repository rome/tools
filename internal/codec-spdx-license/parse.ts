/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	ParserCore,
	SimpleToken,
	StringToken,
	createParser,
	isAlpha,
	isDigit,
} from "@internal/parser-core";
import {getSPDXLicense, licenseNames} from "./index";
import {descriptions, Diagnostics} from "@internal/diagnostics";
import {Number0} from "@internal/ob1";
import {
	ExpressionNode,
	LicenseNode,
	SPDXLicenseParserOptions,
} from "@internal/codec-spdx-license/types";
import {satisfiesSemver, stringifySemver} from "@internal/codec-semver";
import stringify from "@internal/codec-semver/stringify";

//# Tokens
type Tokens = BaseTokens & {
	ParenOpen: SimpleToken<"ParenOpen">;
	ParenClose: SimpleToken<"ParenClose">;
	Plus: SimpleToken<"Plus">;
	And: SimpleToken<"And">;
	With: SimpleToken<"With">;
	Or: SimpleToken<"Or">;
	Word: StringToken<"Word">;
};

function isWordChar(char: string) {
	return (
		isAlpha(char) ||
		isDigit(char) ||
		char === "-" ||
		char === "." ||
		char === "/"
	);
}

type SPDXParserTypes = {
	tokens: Tokens;
	state: {};
	options: SPDXLicenseParserOptions;
	meta: void;
};

type SPDXParser = ParserCore<SPDXParserTypes>;

export interface SPDXLicenseParseResult {
	license: ExpressionNode;
	diagnostics: Diagnostics;
}

const spdxLicenseParser = createParser<SPDXParserTypes>({
	diagnosticLanguage: "spdxlicense",
	ignoreWhitespaceTokens: true,
	tokenize(parser: SPDXParser, index: Number0) {
		const char = parser.getInputCharOnly(index);

		if (char === "+") {
			return parser.finishToken("Plus");
		}

		if (char === "(") {
			return parser.finishToken("ParenOpen");
		}

		if (char === ")") {
			return parser.finishToken("ParenClose");
		}

		if (isWordChar(char)) {
			const [value, end] = parser.readInputFrom(index, isWordChar);

			if (value === "AND") {
				return parser.finishToken("And", end);
			} else if (value === "OR") {
				return parser.finishToken("Or", end);
			} else if (value === "WITH") {
				return parser.finishToken("With", end);
			} else {
				return parser.finishValueToken("Word", value, end);
			}
		}

		return undefined;
	},
});

function parseExpression(parser: SPDXParser): ExpressionNode {
	const startPos = parser.getPosition();
	const startToken = parser.getToken();

	let value;

	switch (startToken.type) {
		case "ParenOpen": {
			parser.nextToken();
			value = parseExpression(parser);
			parser.expectToken("ParenClose");
			break;
		}

		case "Word": {
			value = parseLicense(parser, startToken);
			break;
		}

		case "Or":
		case "And":
			throw parser.unexpected({
				description: descriptions.SPDX.OPERATOR_NOT_BETWEEN_EXPRESSION,
			});

		case "Plus":
			throw parser.unexpected({
				description: descriptions.SPDX.PLUS_NOT_AFTER_LICENSE,
			});

		case "ParenClose":
			throw parser.unexpected({
				description: descriptions.SPDX.UNOPENED_PAREN,
			});

		default:
			throw parser.unexpected();
	}

	// Parse and/or
	const nextToken = parser.getToken();
	switch (nextToken.type) {
		case "Or": {
			parser.nextToken();
			return {
				type: "Or",
				loc: parser.finishLoc(startPos),
				left: value,
				right: parseExpression(parser),
			};
		}

		case "And": {
			parser.nextToken();
			return {
				type: "And",
				loc: parser.finishLoc(startPos),
				left: value,
				right: parseExpression(parser),
			};
		}

		default:
			return value;
	}
}

function isLicenseValid(
	parser: SPDXParser,
	id: string,
): {
	inConfig: boolean;
	satisfiesVersion: boolean;
	packageVersionInConfig: string;
} {
	let inConfig = false;
	let satisfiesVersion = false;
	let packageVersionInConfig = "";

	const {exceptions} = parser.options;
	if (exceptions !== undefined) {
		for (const project of exceptions.projects) {
			const {invalidLicenses} = project.config.dependencies.exceptions;
			const licenses = invalidLicenses.get(id);
			if (licenses === undefined) {
				continue;
			}

			for (const license of licenses) {
				if (license.name !== exceptions.packageName) {
					continue;
				}
				
				satisfiesVersion = satisfiesSemver(
					exceptions.packageVersion,
					license.range,
				);
				inConfig = true;

				// There's at least a version of the dependency that doesn't satisfy the criteria
				// so exit the loop and communicate to the user
				if (!satisfiesVersion) {
					packageVersionInConfig = stringify(license.range);
					break;
				}
			}
		}
	}

	return {inConfig, satisfiesVersion, packageVersionInConfig};
}

function parseLicense(parser: SPDXParser, token: Tokens["Word"]): LicenseNode {
	const startPos = parser.getPosition();
	const nextToken = parser.nextToken();

	// Validate id
	const id = token.value;
	let licenseInfo;

	// Retrieve the license info only if the next token is not a word
	if (nextToken.type !== "Word") {
		licenseInfo = getSPDXLicense(id);
	}

	// If next token is a word,
	let possibleCorrectLicense;
	let endToken = token;
	if (licenseInfo === undefined && nextToken.type === "Word") {
		const words: string[] = [id, nextToken.value];
		while (parser.matchToken("Word")) {
			const token = parser.nextToken();
			if (token.type === "Word") {
				words.push(token.value);
			}
		}
		possibleCorrectLicense = words.join(" ");
		licenseInfo = getSPDXLicense(possibleCorrectLicense);
	}

	// Sometimes licenses will be specified as "Apache 2.0" but what they actually meant was "Apache-2.0"
	// In loose mode, just make it equivalent, otherwise, complain
	if (licenseInfo === undefined && nextToken.type === "Word") {
		if (!possibleCorrectLicense) {
			possibleCorrectLicense = `${id}-${nextToken.value}`;
		}

		const possibleLicenseInfo = getSPDXLicense(possibleCorrectLicense);

		if (possibleLicenseInfo !== undefined) {
			if (parser.options.loose) {
				endToken = nextToken;

				// Just allow it...
				licenseInfo = possibleLicenseInfo;
				parser.nextToken();
			} else {
				parser.unexpectedDiagnostic({
					description: descriptions.SPDX.VALID_LICENSE_WITH_MISSING_DASH(
						possibleCorrectLicense,
					),
					start: parser.getPositionFromIndex(token.start),
					end: parser.getPositionFromIndex(nextToken.end),
				});
			}
		}
	}

	let licenseId: string;

	if (licenseInfo === undefined) {
		const {inConfig, satisfiesVersion, packageVersionInConfig} = isLicenseValid(
			parser,
			possibleCorrectLicense || id,
		);

		if (!satisfiesVersion && !inConfig) {
			const {exceptions} = parser.options;

			if (satisfiesVersion) {
				parser.unexpectedDiagnostic({
					description: descriptions.SPDX.UNKNOWN_LICENSE({
						id: possibleCorrectLicense || id,
						knownLicenses: licenseNames,
						exceptions,
					}),
					start: parser.getPositionFromIndex(token.start),
					end: parser.getPositionFromIndex(endToken.end),
				});
			} else {
				parser.unexpectedDiagnostic({
					description: descriptions.SPDX.UNKNOWN_LICENSE_PRESENT_UNSATISFIED_EXCEPTION({
						id: possibleCorrectLicense || id,
						packageVersionInConfig,
						packageName: exceptions?.packageName ?? "unknown",
						packageVersion: exceptions === undefined
							? "unknown"
							: stringifySemver(exceptions.packageVersion),
					}),
					start: parser.getPositionFromIndex(token.start),
					end: parser.getPositionFromIndex(endToken.end),
				});
			}
		}

		// License has an exception
		licenseId = id;
	} else {
		licenseId = licenseInfo.licenseId;
	}

	// Is this a plus? (wtf is this)
	const plus = parser.eatToken("Plus") !== undefined;

	// Get exception
	let exception;
	if (parser.eatToken("With")) {
		const token = parser.getToken();
		if (token.type === "Word") {
			exception = token.value;
			parser.nextToken();
		} else {
			throw parser.unexpected({
				description: descriptions.SPDX.WITH_RIGHT_LICENSE_ONLY,
			});
		}
	}

	return {
		type: "License",
		loc: parser.finishLoc(startPos),
		id: licenseId,
		exception,
		plus,
	};
}

export function parseSPDXLicense(opts: SPDXLicenseParserOptions): SPDXLicenseParseResult {
	const parser = spdxLicenseParser.create(opts);
	const expr = parseExpression(parser);
	parser.finalize();
	return {
		license: expr,
		diagnostics: parser.getDiagnostics(),
	};
}
