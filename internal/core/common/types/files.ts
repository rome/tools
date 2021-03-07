/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, UIDPath} from "@internal/path";

type FileReferenceBase = {
	uid: UIDPath;
	real: AbsoluteFilePath;
	remote?: boolean;
};

type ProjectFileReference = FileReferenceBase & {
	project: number;
};
type ManifestFileReference = FileReferenceBase & {
	manifest: number;
};

export type FileReference = ProjectFileReference | ManifestFileReference;
