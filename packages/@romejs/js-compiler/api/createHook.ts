/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from '@romejs/js-compiler';

export type HookDescriptor<State, CallArg, CallReturn> = {
  name: string;
  initialState: State extends void ? never : State;
  call?: (path: Path, state: State, arg: CallArg) => {
    bubble?: boolean;
    value: CallReturn;
    state: State;
  };
  exit?: (path: Path, state: State) => TransformExitResult;
};

// rome-suppress lint/noExplicitAny
export type AnyHookDescriptor = HookDescriptor<any, any, any>;

export type HookInstance = {
  // rome-suppress lint/noExplicitAny
  state: any;
  // rome-suppress lint/noExplicitAny
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
