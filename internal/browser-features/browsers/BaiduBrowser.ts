import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class BaiduBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "baidu",
			version
		})
	}
}
