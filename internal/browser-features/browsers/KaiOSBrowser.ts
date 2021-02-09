import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class KaiOSBrowser extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "kaios",
			version
		})
	}
}
