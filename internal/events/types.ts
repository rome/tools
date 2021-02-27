/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {StructuredError} from "@internal/errors";
import {AnyMarkups} from "@internal/markup";
import {RSERObject, RSERValue} from "@internal/codec-binary-serial";
import {Dict, VoidCallback} from "@internal/typescript-helpers";
import {
	BridgeEventBidirectional,
	BridgeEventCallOnly,
	BridgeEventListenOnly,
} from "./BridgeEvent";
import {BridgeFactories} from "./createBridge";
import Bridge from "./Bridge";
import { Resource } from "@internal/resources";
import { Duration } from "@internal/numbers";

export type EventCallback<Param, Ret> = (
	param: Param,
	resource: Resource,
) => Ret | Promise<Ret>;

export type AnyBridge = Bridge<{}, {}, {}>;

// rome-ignore lint/js/noUnusedVariables(Ret) lint/js/noUnusedVariables(Param): Only care about creating a generic
export type BridgeEventDeclaration<
	Param extends RSERValue,
	Ret extends RSERValue
> = {};

export type BridgeEventsDeclaration = Dict<BridgeEventDeclaration<
	RSERValue,
	RSERValue
>>;

type ExtractEventTypes<Type> = Type extends BridgeEventDeclaration<
	infer Param,
	infer Ret
>
	? [Param, Ret]
	: never;

export type BridgeEventsDeclarationToInstances<
	ListenEvents extends BridgeEventsDeclaration,
	CallEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
> = {
	[Key in keyof ListenEvents]: BridgeEventListenOnly<
		ExtractEventTypes<ListenEvents[Key]>[0],
		ExtractEventTypes<ListenEvents[Key]>[1]
	>
} & {
	[Key in keyof CallEvents]: BridgeEventCallOnly<
		ExtractEventTypes<CallEvents[Key]>[0],
		ExtractEventTypes<CallEvents[Key]>[1]
	>
} & {
	[Key in keyof SharedEvents]: BridgeEventBidirectional<
		ExtractEventTypes<SharedEvents[Key]>[0],
		ExtractEventTypes<SharedEvents[Key]>[1]
	>
};

export type BridgeClient<Factories> = Factories extends BridgeFactories<
	infer ClientEvents,
	infer ServerEvents,
	infer SharedEvents
>
	? Bridge<ClientEvents, ServerEvents, SharedEvents>
	: never;

export type BridgeServer<Factories> = Factories extends BridgeFactories<
	infer ClientEvents,
	infer ServerEvents,
	infer SharedEvents
>
	? Bridge<ServerEvents, ClientEvents, SharedEvents>
	: never;

export type BridgeType = "server" | "client";

export type BridgeDefinition<
	ClientEvents extends BridgeEventsDeclaration,
	ServerEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
> = {
	debugName: string;
	client: ClientEvents;
	server: ServerEvents;
	shared: SharedEvents;
	init?: BridgeInitCallback<SharedEvents>;
};

export type BridgeInitCallback<SharedEvents extends BridgeEventsDeclaration> = (
	bridge: Bridge<{}, {}, SharedEvents>,
) => void;

export type EventOptions = {
	displayName?: string;
	requiredSubscriptionResource?: boolean;
	onSubscriptionChange?: VoidCallback;
	unique?: boolean;
	serial?: boolean;
};

export type EventSubscriptionOptions = {};

export type BridgeHeartbeatExceededOptions = {
	summary: AnyMarkups;
	attempts: number;
	totalTime: Duration;
};

export enum BridgeMessageCodes {
	CLIENT_HANDSHAKE,
	SERVER_HANDSHAKE,
	SUBSCRIBED,
	UNSUBSCRIBED,
	HEARTBEAT,
	CALL,
	PRIORITY_CALL,
	SEND,
	RESPONSE_SUCCESS,
	RESPONSE_ERROR_CUSTOM,
	RESPONSE_ERROR_NATIVE,
}

export type BridgeHandshakeMessage = [BridgeMessageCodes.CLIENT_HANDSHAKE, undefined | Duration, Set<number>, Map<number, string>] | [BridgeMessageCodes.SERVER_HANDSHAKE, undefined | Duration, Set<number>];
export type BridgeSubscriptionsMessage = [BridgeMessageCodes.SUBSCRIBED | BridgeMessageCodes.UNSUBSCRIBED, number];
export type BridgeRequestCallMessage = [BridgeMessageCodes.CALL | BridgeMessageCodes.PRIORITY_CALL, number, number, RSERValue] | [BridgeMessageCodes.CALL | BridgeMessageCodes.PRIORITY_CALL, number, number];
export type BridgeRequestSendMessage = [BridgeMessageCodes.SEND, number, RSERValue];
export type BridgeSuccessResponseMessage = [BridgeMessageCodes.RESPONSE_SUCCESS, number, RSERValue];
export type BridgeNativeErrorResponseMessage = [BridgeMessageCodes.RESPONSE_ERROR_NATIVE, number, Error];
export type BridgeCustomErrorResponseMessage = [BridgeMessageCodes.RESPONSE_ERROR_CUSTOM, number, StructuredError, RSERObject];
export type BridgeHeartbeatMessage = [BridgeMessageCodes.HEARTBEAT];

export type BridgeRequestMessage = BridgeRequestCallMessage | BridgeRequestSendMessage;

export type BridgeErrorResponseMessage = BridgeNativeErrorResponseMessage | BridgeCustomErrorResponseMessage;

export type BridgeResponseMessage =
	| BridgeSuccessResponseMessage
	| BridgeErrorResponseMessage;

export type BridgeMessage =
  | BridgeHeartbeatMessage
	| BridgeHandshakeMessage
	| BridgeSubscriptionsMessage
	| BridgeRequestMessage
	| BridgeResponseMessage;

	export type BridgeErrorDetails = {
		errorType: "custom";
		value: StructuredError;
		metadata: RSERObject;
	} | {
		errorType: "native";
		value: Error;
	};
