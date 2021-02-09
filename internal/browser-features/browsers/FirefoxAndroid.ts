import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class FirefoxAndroid extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "and_ff",
			version
		})
	}
}
