import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class AndroidBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "android",
			version
		})
	}
}
