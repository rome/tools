/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import { test } from "rome";
import { testLintMultiple } from "../testHelpers";

test("accessible emoji", async (t) => {
  await testLintMultiple(
    t,
    [
      // INVALID
      '<span>🐼</span>',
      '<i role="img" aria-label="Panda">🐼</i>',
      // VALID
      '<span role="img" aria-label="Panda">🐼</span>',
      '<span role="img" aria-labelledby="panda1">🐼</span>',
      '<span role="img" aria-label="Snowman">&#9731;</span>',
    ],
    { category: "lint/accessibleEmoji" }
  );
});
