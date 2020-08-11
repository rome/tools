/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {StructuredError} from "@internal/v8";
import {AnyMarkups} from "@internal/markup";
import {RSERObject, RSERValue} from "@internal/codec-binary-serial";

export type BridgeCreatorOptions = {
	type: BridgeType;
	onSendMessage?: (msg: BridgeMessage) => void;
};

export type BridgeType = "server" | "client" | "server&client";

export type BridgeOptions = BridgeCreatorOptions & {
	sendMessage: (msg: BridgeMessage) => void;
};

export type EventOptions = {
	name: string;
	unique?: boolean;
	serial?: boolean;
};

export type EventSubscription = {
	unsubscribe: () => Promise<void>;
};

export type EventSubscriptions = Array<EventSubscription>;

export type BridgeHeartbeatExceededOptions = {
	summary: AnyMarkups;
	iterations: number;
	totalTime: number;
};

export type BridgeHandshakeMessage = {
	type: "handshake";
	first: boolean;
	subscriptions: Array<string>;
};

export type BridgeSubscriptionsMessage = {
	type: "subscriptions";
	names: Array<string>;
};

export type BridgeRequestMessage = {
	id?: number;
	event: string;
	param: RSERValue;
	type: "request";
	priority: boolean;
};

export type BridgeSuccessResponseMessage = {
	id: number;
	event: string;
	value: RSERValue;
	type: "response";
	responseStatus: "success";
};

export type BridgeErrorResponseDetails = {
	value: StructuredError;
	metadata: RSERObject;
};

export type BridgeErrorResponseMessage = BridgeErrorResponseDetails & {
	id: number;
	event: string;
	type: "response";
	responseStatus: "error";
};

export type BridgeResponseMessage =
	| BridgeSuccessResponseMessage
	| BridgeErrorResponseMessage;

export type BridgeMessage =
	| BridgeHandshakeMessage
	| BridgeSubscriptionsMessage
	| BridgeRequestMessage
	| BridgeResponseMessage;
