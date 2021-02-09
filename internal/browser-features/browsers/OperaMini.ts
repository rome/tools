import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class OperaMini extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "op_mini",
			version
		})
	}
}
