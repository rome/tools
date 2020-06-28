/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/compiler";

export type HookCallReturn<CallReturn, State> = {
	bubble?: boolean;
	value: CallReturn;
	state: State;
};

export type HookDescriptor<State, CallArg, CallReturn> = {
	name: string;
	initialState: State extends void ? never : State;
	call?: (
		path: Path,
		state: State,
		arg: CallArg,
	) => HookCallReturn<CallReturn, State>;
	exit?: (path: Path, state: State) => TransformExitResult;
};

// rome-ignore lint/js/noExplicitAny
export type AnyHookDescriptor = HookDescriptor<any, any, any>;

export type HookInstance = {
	// rome-ignore lint/js/noExplicitAny
	state: any;
	// rome-ignore lint/js/noExplicitAny
	descriptor: HookDescriptor<any, any, any>;
};

export default function createHook<
	State = void,
	CallArg = void,
	CallReturn = void
>(
	descriptor: HookDescriptor<State, CallArg, CallReturn>,
): HookDescriptor<State, CallArg, CallReturn> {
	return descriptor;
}
