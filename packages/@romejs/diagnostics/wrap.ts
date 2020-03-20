/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostics, DiagnosticOrigin} from './types';
import {addOriginsToDiagnostics} from './derive';
import {getDiagnosticsFromError} from './errors';

type WrapResult<T> =
  | {
    readonly value: T;
    readonly diagnostics: undefined;
  }
  | {
    readonly value: undefined;
    readonly diagnostics: PartialDiagnostics;
  };

export async function catchDiagnostics<T>(
  origin: DiagnosticOrigin,
  promise: () => Promise<T>,
): Promise<WrapResult<T>> {
  try {
    const value = await promise();

    return {value, diagnostics: undefined};
  } catch (err) {
    const diagnostics = getDiagnosticsFromError(err);

    if (diagnostics) {
      return {
        value: undefined,
        diagnostics: addOriginsToDiagnostics([origin], diagnostics),
      };
    } else {
      throw err;
    }
  }
}
