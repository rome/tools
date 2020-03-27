/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, createAbsoluteFilePath} from '@romejs/path';

export type FileReference = {
  project: number;
  manifest: undefined | number;
  uid: string;
  real: AbsoluteFilePath;
  remote: boolean;
  sourceText?: string;
};

export type JSONFileReference = Omit<FileReference, 'real'> & {real: string};

export function convertTransportFileReference(
  ref: JSONFileReference,
): FileReference {
  return {
    ...ref,
    real: createAbsoluteFilePath(ref.real),
  };
}
