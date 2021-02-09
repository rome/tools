import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class SamsungInternet extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "samsung",
			version
		})
	}

	public static getVersionImplementation(version: string): SamsungInternet {
		return new SamsungInternet({version});
	}
}
