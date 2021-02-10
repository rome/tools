import {Browser, BrowserProps} from "@internal/browser-features/Browser";

export default class Edge extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "edge",
			version: props?.version,
		});
	}
}
