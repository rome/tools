import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class OperaMini extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "op_mini",
			version: props?.version,
		});
	}
}
