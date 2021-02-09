import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class Safari extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "safari",
			version
		})
	}

	public static getVersionImplementation(version: string): Safari {
		return new Safari({version});
	}
}
