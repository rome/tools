import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class KaiOSBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "kaios",
			version
		})
	}

	public static getVersionImplementation(version: string): KaiOSBrowser {
		return new KaiOSBrowser({version});
	}
}
