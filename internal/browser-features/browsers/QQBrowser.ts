import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class QQBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "and_qq",
			version
		})
	}

	public static getVersionImplementation(version: string): QQBrowser {
		return new QQBrowser({version});
	}
}
