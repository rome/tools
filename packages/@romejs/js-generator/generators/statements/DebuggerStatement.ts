/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';

export default function DebuggerStatement(generator: Generator) {
  generator.word('debugger');
  generator.semicolon();
}
