import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class Opera extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "opera",
			version
		})
	}

	public static getVersionImplementation(version: string): Opera {
		return new Opera({version});
	}
}
