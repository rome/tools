export const PROJECT_CONFIG_PACKAGE_JSON_FIELD = "rome";
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

// Add dot versions
for (const basename of PROJECT_CONFIG_WARN_FILENAMES) {
	if (basename[0] !== ".") {
		PROJECT_CONFIG_WARN_FILENAMES.push(`.${basename}`);
	}
}
for (const filename of PROJECT_CONFIG_FILENAMES.slice()) {
	PROJECT_CONFIG_FILENAMES.push(`.${filename}`);
}
