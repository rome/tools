import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class SafariIOS extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "ios_saf",
			version
		})
	}
}
