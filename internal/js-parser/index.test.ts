/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {declareParserTests} from "@internal/test-helpers";

const promise = declareParserTests();

// @ts-ignore Doesn't support top-level await lol
await promise;
