import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Opera extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "opera",
			version
		})
	}
}
