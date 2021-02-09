import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Firefox extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "firefox",
			version
		})
	}
}
