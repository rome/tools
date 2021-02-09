import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class AndroidBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "android",
			version
		})
	}

	public static getVersionImplementation(version: string): AndroidBrowser {
		return new AndroidBrowser({version});
	}
}
