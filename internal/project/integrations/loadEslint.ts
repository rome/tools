import {consumeUnknown} from "@internal/consume";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {IntegrationEslintConfig, arrayOfStrings} from "@internal/project";
import {json} from "@internal/codec-config";

export function loadEslint(file: string): Partial<IntegrationEslintConfig> {
	const config: Partial<IntegrationEslintConfig> = {};
	const data = json.parse({input: file});
	const eslint = consumeUnknown(data, DIAGNOSTIC_CATEGORIES.eslint, "json");

	if (eslint.has("globInputPaths")) {
		config.globInputPaths = eslint.get("globInputPaths").asBoolean();
	}
	if (eslint.has("extensions")) {
		config.extensions = arrayOfStrings(eslint.get("extensions"));
	}
	if (eslint.has("fix")) {
		config.fix = eslint.get("fix").asBoolean();
	}
	if (eslint.has("rulePaths")) {
		config.rulePaths = arrayOfStrings(eslint.get("rulePaths"));
	}

	return config;
}
