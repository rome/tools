import {consumeUnknown} from "@internal/consume";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {IntegrationPrettierConfig} from "@internal/project";
import {json, json5} from "@internal/codec-config";

export function loadPrettier(
	file: string,
	extension: string,
): Partial<IntegrationPrettierConfig> {
	const config: Partial<IntegrationPrettierConfig> = {};
	let data: unknown;
	// NOTE: we only support json for now
	if (extension === "json5") {
		data = json5.parse({input: file});
	} else {
		data = json.parse({input: file});
	}
	const prettier = consumeUnknown(data, DIAGNOSTIC_CATEGORIES.prettier, "json");

	if (prettier.has("printWidth")) {
		config.printWidth = prettier.get("printWidth").asNumber();
	}
	if (prettier.has("tabWidth")) {
		config.tabWidth = prettier.get("tabWidth").asNumber();
	}
	if (prettier.has("useTabs")) {
		config.useTabs = prettier.get("useTabs").asBoolean();
	}
	if (prettier.has("semi")) {
		config.semi = prettier.get("semi").asBoolean();
	}
	if (prettier.has("singleQuote")) {
		config.singleQuote = prettier.get("singleQuote").asBoolean();
	}

	return config;
}
