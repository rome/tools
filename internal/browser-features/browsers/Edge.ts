import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class Edge extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "edge",
			version
		})
	}

	public static getVersionImplementation(version: string): Edge {
		return new Edge({version});
	}
}
