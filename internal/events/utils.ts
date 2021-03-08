/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DIAGNOSTIC_CATEGORIES,
	isDiagnosticsErrorOfCategory,
} from "@internal/diagnostics";
import {
	BridgeMessage,
	BridgeMessageCodes,
	BridgeResponseMessage,
} from "./types";

export function isBridgeEndDiagnosticsError(err: Error): boolean {
	return (
		isDiagnosticsErrorOfCategory(
			err,
			DIAGNOSTIC_CATEGORIES["bridge/disconnected"],
		) ||
		isDiagnosticsErrorOfCategory(err, DIAGNOSTIC_CATEGORIES["bridge/closed"])
	);
}

export function isBridgeResponseMessage(
	msg: BridgeMessage,
): msg is BridgeResponseMessage {
	return (
		msg[0] === BridgeMessageCodes.RESPONSE_SUCCESS ||
		msg[0] === BridgeMessageCodes.RESPONSE_ERROR_CUSTOM ||
		msg[0] === BridgeMessageCodes.RESPONSE_ERROR_NATIVE
	);
}
