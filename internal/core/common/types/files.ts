/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, RelativeFilePath, UIDPath} from "@internal/path";

export type FileReference = {
	project: number;
	manifest: undefined | number;
	uid: UIDPath;
	relative: RelativeFilePath;
	real: AbsoluteFilePath;
	remote: boolean;
};
