import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class ChromeAndroid extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "and_chr",
			version
		})
	}

	public static getVersionImplementation(version: string): ChromeAndroid {
		return new ChromeAndroid({version});
	}
}
