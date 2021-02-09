import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Safari extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "safari",
			version
		})
	}
}
