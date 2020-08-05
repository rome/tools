/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Bridge} from "@internal/events";
import {WebServerClient, WebServerRequest} from "../../server/web";

export default class WebBridge extends Bridge {
	public requests = this.createEvent<
		{
			requests: Array<WebServerRequest>;
			clients: Array<WebServerClient>;
		},
		void
	>({
		name: "WebBridge.requests",
		direction: "server->client",
	});
}
