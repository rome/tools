import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class BlackberryBrowser extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "bb",
			version: props?.version,
		});
	}
}
