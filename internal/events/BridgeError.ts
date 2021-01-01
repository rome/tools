/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyBridge} from "./types";

export default class BridgeError extends Error {
	constructor(message: string, bridge: AnyBridge) {
		super(message);
		this.bridge = bridge;
	}

	public bridge: AnyBridge;
}
