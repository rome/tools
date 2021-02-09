import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class SafariIOS extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "ios_saf",
			version
		})
	}

	public static getVersionImplementation(version: string): SafariIOS {
		return new SafariIOS({version});
	}
}
