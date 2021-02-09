import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class BlackberryBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "bb",
			version
		})
	}

	public static getVersionImplementation(version: string): BlackberryBrowser {
		return new BlackberryBrowser({version});
	}
}
