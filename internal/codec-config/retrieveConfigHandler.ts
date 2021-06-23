import {ConfigHandler, ConfigType} from "@internal/codec-config/types";
import {json, toml} from "@internal/codec-config";

export default function retrieveConfigHandler(
	extension: ConfigType,
): ConfigHandler {
	let manifestHandler: ConfigHandler | undefined = undefined;
	// TODO: add yaml and toml to config
	// if (configType === "yaml") {
	// 	manifestHandler = yaml;
	// 	fileExtension = "yaml";
	// } else
	if (extension === "toml") {
		manifestHandler = toml;
	} else if (extension === "json") {
		manifestHandler = json;
	}

	if (!manifestHandler) {
		manifestHandler = toml;
	}

	return manifestHandler;
}
