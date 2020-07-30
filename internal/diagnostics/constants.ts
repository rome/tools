/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticAdviceItem} from "./types";
import {markup} from "@internal/markup";

export const INTERNAL_ERROR_LOG_ADVICE: DiagnosticAdviceItem = {
	type: "log",
	category: "warn",
	text: markup`This diagnostic was derived from an internal Rome error. Possible bug.`,
};
