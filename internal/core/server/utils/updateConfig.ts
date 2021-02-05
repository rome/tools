import {ConfigCommentMap} from "@internal/codec-config";
import {writeFile} from "@internal/fs";
import {ConfigHandler} from "@internal/codec-config/types";
import {AbsoluteFilePath} from "@internal/path";
import {ProjectConfig} from "@internal/project";
import {DeepPartial} from "@internal/typescript-helpers";

interface UpdateConfig {
	config: Partial<ProjectConfig>;
	configPath: AbsoluteFilePath;
	partial?: DeepPartial<ProjectConfig>;
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
	await writeFile(
		configPath,
		configHandler.stringify(finalConfig, comments) + "\n",
	);
}
