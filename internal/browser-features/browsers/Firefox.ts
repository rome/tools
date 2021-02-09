import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class Firefox extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "firefox",
			version
		})
	}

	public static getVersionImplementation(version: string): Firefox {
		return new Firefox({version});
	}
}
