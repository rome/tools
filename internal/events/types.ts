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

export type EventCallback<Param, Ret> = (
	param: Param,
	subscription: EventSubscription,
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
	name: string;
	displayName?: string;
	onSubscriptionChange?: VoidCallback;
	unique?: boolean;
	serial?: boolean;
};

export type EventSubscription = {
	unsubscribe: () => Promise<void>;
};

export type EventSubscriptions = EventSubscription[];

export type BridgeHeartbeatExceededOptions = {
	summary: AnyMarkups;
	iterations: number;
	totalTime: number;
};

export type BridgeHandshakeMessage = {
	type: "handshake";
	subscriptions: Set<string>;
};

export type BridgeSubscriptionsMessage = {
	type: "subscriptions";
	names: Set<string>;
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
