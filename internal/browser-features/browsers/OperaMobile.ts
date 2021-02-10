import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class OperaMobile extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "op_mob",
			version: props?.version,
		});
	}
}
