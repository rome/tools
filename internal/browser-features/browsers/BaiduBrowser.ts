import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class BaiduBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "baidu",
			version
		})
	}

	public static getVersionImplementation(version: string): BaiduBrowser {
		return new BaiduBrowser({version});
	}
}
