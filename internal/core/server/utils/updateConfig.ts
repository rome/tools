import {ConfigCommentMap} from "@internal/codec-config";
import {ConfigHandler} from "@internal/codec-config/types";
import {AbsoluteFilePath} from "@internal/path";
import {RawUserProjectConfig} from "@internal/project";
import {DeepPartial} from "@internal/typescript-helpers";

interface UpdateConfig {
	config: Partial<RawUserProjectConfig>;
	configPath: AbsoluteFilePath;
	partial?: DeepPartial<RawUserProjectConfig>;
	configHandler: ConfigHandler;
	comments?: ConfigCommentMap;
}

export default async function updateConfig(
	{config, configPath, configHandler, partial = {}, comments = new Map()}: UpdateConfig,
): Promise<void> {
	// Update it on disk
	const finalConfig = {
		...config,
		...partial,
	};
	await configPath.writeFile(
		configHandler.stringify(finalConfig, comments) + "\n",
	);
}
