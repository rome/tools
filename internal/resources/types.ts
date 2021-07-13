import {AsyncVoidCallback, RequiredProps} from "@internal/typescript-helpers";
import Resource from "./Resource";

export type FactoryResourceOptions = {
	name?: string;
	optional?: boolean;
	initialResources?: ResourcesContainer[];
};

export type FactoryResourceContainerOptions = FactoryResourceOptions & {
	finalize?: AsyncVoidCallback;
};

export type ResourceOptions = RequiredProps<
	FactoryResourceContainerOptions,
	"name"
> & {
	getDetails: ResourceGetDetails;
	release?: AsyncVoidCallback;
};

export type ResourceGetDetails = () => ResourceDetails;

// TODO: Get rid of this probably
export type ResourcesContainer =
	| Resource
	| {
			resources: undefined | Resource;
		};

export type ResourceTreeEntry = {
	details: ResourceDetails;
	children: ResourceTree;
};

export type ResourceTree = Map<Resource, ResourceTreeEntry>;

export type ProcessResourceDetails = {
	type: "process";
	self: boolean;
	pid: number;
	tid: number;
	command: string;
	args: string[];
};

export type WorkerThreadResourceDetails = {
	type: "worker";
	// There's no public Worker properties...
};

export type SocketResourceDetails = {
	type: "socket";
	localAddress: string;
	localPort: number;
	remoteAddress: undefined | string;
	remoteFamily: undefined | string;
	remotePort: undefined | number;
};

export type WebSocketResourceDetails = {
	type: "websocket";
	url: string;
};

export type ServerResourceDetails = {
	type: "server";
	address: string;
	family: string;
	port: number;
};

export type TimerResourceDetails = {
	type: "timer";
	kind: "interval" | "timeout" | "unknown";
	delay: undefined | number;
};

export type CallbackResourceDetails = {
	type: "callback";
};

export type RootResourceDetails = {
	type: "root";
};

export type ContainerResourceDetails = {
	type: "container";
};

export type ResourceDetails =
	| RootResourceDetails
	| ProcessResourceDetails
	| CallbackResourceDetails
	| WorkerThreadResourceDetails
	| WebSocketResourceDetails
	| SocketResourceDetails
	| ServerResourceDetails
	| TimerResourceDetails
	| ContainerResourceDetails;
