/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */


import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

/*
 * plain-English description of the following regexp:
 * 1. `^` fix the match at the beginning of the string
 * 2. `([^\\[]|\\.|\[([^\\\]]|\\.)+\])*`: regexp contents; 0 or more of the following
 * 2.0. `[^\\[]`: any character that's not a `\` or a `[` (anything but escape sequences and character classes)
 * 2.1. `\\.`: an escape sequence
 * 2.2. `\[([^\\\]]|\\.)+\]`: a character class that isn't empty
 * 3. `$`: fix the match at the end of the string
 */

const regex = /^([^\\[]|\\.|\[([^\\\]]|\\.)+\])*$/u;

export default {
  name: 'noEmptyCharacterClass',
  enter(path: Path): AnyNode {
    const {context, node} = path;

		if (node.type === "RegExpLiteral" && !regex.test(node.pattern)) {
			context.addNodeDiagnostic(node, {
        category: 'lint/noEmptyCharacterClass',
        message:
          'Empty character classes in regular expressions are not allowed',
      });
		}
    
    return node;
  },
};
