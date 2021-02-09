import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class ChromeAndroid extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "and_chr",
			version
		})
	}
}
