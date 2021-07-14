import {ConfigHandler, ConfigType} from "@internal/codec-config/types";
import {json, toml} from "@internal/codec-config";

export default function retrieveConfigHandler(
	extension: ConfigType,
): ConfigHandler {
	switch (extension) {
		case "toml":
			return toml;
		case "json":
			return json;
		default:
			return json;
	}
}
