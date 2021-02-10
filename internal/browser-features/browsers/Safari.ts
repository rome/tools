import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Safari extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "safari",
			version: props?.version,
		});
	}
}
