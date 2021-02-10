import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Chrome extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "chrome",
			version: props?.version,
		});
	}
}
