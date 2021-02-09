import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export class OperaMobile extends Browser {
	constructor({version}: Pick<BrowserProps, "version">) {
		super({
			id: "op_mob",
			version
		})
	}

	public static getVersionImplementation(version: string): OperaMobile {
		return new OperaMobile({version});
	}
}
