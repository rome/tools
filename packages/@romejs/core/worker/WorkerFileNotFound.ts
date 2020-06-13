import {AbsoluteFilePath} from "@romejs/path";

export class WorkerFileNotFound extends Error {
	constructor(path: AbsoluteFilePath) {
		super();
		this.name = "WorkerFileNotFound";
		this.path = path;
	}

	path: AbsoluteFilePath;
}
