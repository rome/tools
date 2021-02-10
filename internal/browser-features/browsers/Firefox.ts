import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Firefox extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "firefox",
			version: props?.version,
		});
	}
}
