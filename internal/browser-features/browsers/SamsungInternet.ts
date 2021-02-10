import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class SamsungInternet extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "samsung",
			version: props?.version,
		});
	}
}
