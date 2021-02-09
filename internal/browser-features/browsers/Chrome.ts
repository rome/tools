import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class Chrome extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "chrome",
			version
		})
	}

	public static getVersionImplementation(version: string): Chrome {
		return new Chrome({version});
	}
}
