import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Edge extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "edge",
			version
		})
	}
}
