/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TransformExitResult} from '@romejs/js-compiler';
import {Path} from '@romejs/js-compiler';

export type HookDescriptor<State, CallArg, CallReturn> = {
  name: string;
  initialState: State extends void ? never : State;
  call?: (
    path: Path,
    state: State,
    arg: CallArg,
  ) => {
    bubble?: boolean;
    value: CallReturn;
    state: State;
  };
  exit?: (path: Path, state: State) => TransformExitResult;
};

export type AnyHookDescriptor = HookDescriptor<any, any, any>;

export type HookInstance = {
  state: any;
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
