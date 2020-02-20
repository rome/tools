/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TestAPI} from '@romejs/core';

export type TestName = string | Array<string>;

declare const __ROME__TEST_OPTIONS__: GlobalTestOptions;

export type GlobalTestOptions =
  | undefined
  | {
      dirname?: string;
      register?: (
        err: Error,
        opts: import('@romejs/test').TestOptions,
        callback?: import('@romejs/test').TestCallback,
      ) => void;
    };

type NamelessTestOptions = {
  timeout?: number;
  only?: boolean;
};

export type TestCallback = (t: TestAPI) => void | undefined | Promise<void>;

export type TestOptions = NamelessTestOptions & {
  name: TestName;
};

type TestArg = TestName | NamelessTestOptions | TestCallback | undefined;

const testOptions: NonNullable<GlobalTestOptions> =
  __ROME__TEST_OPTIONS__ === undefined ? {} : __ROME__TEST_OPTIONS__;

export const dirname =
  testOptions.dirname === undefined ? '' : testOptions.dirname;

function registerTest(
  callsiteError: Error,
  opts: TestOptions,
  callback: undefined | TestCallback,
) {
  const register = testOptions.register;

  if (typeof register !== 'function') {
    throw new Error('Test harness does not exist');
  }

  register(callsiteError, opts, callback);
}

function isOptionsObject(arg: TestArg): arg is NamelessTestOptions {
  return typeof arg === 'object' && arg != null && !Array.isArray(arg);
}

function splitArgs(
  args: TestRegisterFunctionArgs,
): {
  options: TestOptions;
  callback: undefined | TestCallback;
} {
  const name: TestName = args[0];
  if (typeof name !== 'string' && !Array.isArray(name)) {
    throw new Error('Expected test name to be a string or an array of strings');
  }
  args.shift();

  let foundOptions;
  let options: NamelessTestOptions = {};
  let callback;

  // Try callback which will always be at the end
  const callbackOrOpts = args.pop();
  if (typeof callbackOrOpts === 'function' || callbackOrOpts === undefined) {
    callback = callbackOrOpts;
  } else if (isOptionsObject(callbackOrOpts)) {
    options = callbackOrOpts;
    foundOptions = true;
  } else {
    throw new Error('Expected to find callback or options at the end');
  }

  // Try options which should be in the middle position
  if (!foundOptions && args.length > 0) {
    const maybeOptions = args.pop();
    if (isOptionsObject(maybeOptions)) {
      options = maybeOptions;
    } else {
      throw new Error('Expected to find test options');
    }
  }

  if (args.length > 0) {
    throw new Error('Expected to have exhausted test register arguments');
  }

  return {
    options: {
      ...options,
      name,
    },
    callback,
  };
}

export * from './fixtures';

type TestRegisterFunctionArgs =
  | [TestName]
  | [TestName, TestCallback]
  | [TestName, NamelessTestOptions, TestCallback];

type TestRegisterFunction = (...args: TestRegisterFunctionArgs) => void;

const test: {
  (...args: TestRegisterFunctionArgs): void;
  skip: TestRegisterFunction;
  only: TestRegisterFunction;
} = function(...args: TestRegisterFunctionArgs) {
  const {options, callback} = splitArgs(args);
  registerTest(new Error(), options, callback);
};

test.skip = function(...args: TestRegisterFunctionArgs) {
  const {options} = splitArgs(args);
  registerTest(new Error(), options, undefined);
};

test.only = function(...args: TestRegisterFunctionArgs) {
  const {options, callback} = splitArgs(args);
  registerTest(
    new Error(),
    {
      ...options,
      only: true,
    },
    callback,
  );
};

export default test;
