import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Chrome extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "chrome",
			version
		})
	}
}
