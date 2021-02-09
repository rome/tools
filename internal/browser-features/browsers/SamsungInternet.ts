import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class SamsungInternet extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "samsung",
			version
		})
	}
}
