import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class FirefoxAndroid extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "and_ff",
			version
		})
	}

	public static getVersionImplementation(version: string): FirefoxAndroid {
		return new FirefoxAndroid({version});
	}
}
