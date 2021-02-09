import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class QQBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "and_qq",
			version
		})
	}
}
