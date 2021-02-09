import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class BlackberryBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "bb",
			version
		})
	}
}
