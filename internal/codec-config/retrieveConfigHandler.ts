import {ConfigHandler, ConfigType} from "@internal/codec-config/types";
import {json, toml} from "@internal/codec-config";

export default function retrieveConfigHandler(
	extension: ConfigType,
): ConfigHandler {
	let manifestHandler: ConfigHandler | undefined = undefined;

	if (extension === "toml") {
	 	manifestHandler = toml;
	}

	if (extension === "json") {
		manifestHandler = json;
	}

	if (manifestHandler === undefined) {
		manifestHandler = json;
	}

	return manifestHandler;
}
