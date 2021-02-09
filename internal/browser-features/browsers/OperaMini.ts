import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class OperaMini extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "op_mini",
			version
		})
	}

	public static getVersionImplementation(version: string): OperaMini {
		return new OperaMini({version});
	}
}
