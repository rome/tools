import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class KaiOSBrowser extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "kaios",
			version: props?.version,
		});
	}
}
