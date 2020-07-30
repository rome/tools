import {
	AbsoluteFilePathSet,
	HOME_PATH,
	createAbsoluteFilePath,
} from "@internal/path";

export const PROJECT_CONFIG_PACKAGE_JSON_FIELD = "rome";
export const PROJECT_CONFIG_DIRECTORY = ".config";
export const PROJECT_CONFIG_FILENAMES: Array<string> = [
	"rome.json",
	"rome.rjson",
];
export const PROJECT_CONFIG_WARN_FILENAMES: Array<string> = [
	"romeconfig",
	"romerc",
	"rome.son",
	"rome.config.ts",
	"rome.config.js",
	"rome.config.json",
	"rome.config.rjson",
	"rome.config.son",
];

// Creating or loading projects from these folders is always a mistake
// We also disallow any roots, check is in ProjectManager
export const PROJECT_CONFIG_SENSITIVE_DIRECTORIES: AbsoluteFilePathSet = new AbsoluteFilePathSet([
	HOME_PATH,
	HOME_PATH.append("Downloads"),
	HOME_PATH.append("Documents"),
	HOME_PATH.append("Desktop"),
	HOME_PATH.append("Library"),
	createAbsoluteFilePath("/root"),
	createAbsoluteFilePath("C:/Windows"),
	createAbsoluteFilePath("C:/Windows/system32"),
]);

// Add dot versions
for (const basename of PROJECT_CONFIG_WARN_FILENAMES) {
	if (basename[0] !== ".") {
		PROJECT_CONFIG_WARN_FILENAMES.push(`.${basename}`);
	}
}
for (const filename of PROJECT_CONFIG_FILENAMES.slice()) {
	PROJECT_CONFIG_FILENAMES.push(`.${filename}`);
}
