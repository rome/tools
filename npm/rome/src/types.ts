import { CurrentFile } from "./index";

export interface FormatFileIntern {
	fileUpdated: boolean;
	currentFile: CurrentFile;
	range?: {
		start: number,
		end: number
	},
	content: string;
	debug: boolean;
}
