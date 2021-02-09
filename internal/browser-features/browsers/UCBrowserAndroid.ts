import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class UCBrowserAndroid extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "and_uc",
			version
		})
	}

	public static getVersionImplementation(version: string): UCBrowserAndroid {
		return new UCBrowserAndroid({version});
	}
}
