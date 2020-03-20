/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import prettyFormat from '@romejs/pretty-format';
import {escapeMarkup} from '@romejs/string-markup';

const formatOpts = {
  maxDepth: 5,
};

export function interpolate(msg: string, args: Array<unknown>): string {
  let argIndex: number = 0;

  // replace %s in the message with each argument
  let interpolated: string = msg.replace(/%s/g, () => {
    return prettyFormat(args[argIndex++], formatOpts);
  });

  // add on all other arguments to the end, separate with spaces
  if (argIndex < args.length) {
    interpolated += ' ';
    interpolated += args.slice(argIndex).map((arg) => 
      escapeMarkup(prettyFormat(arg, formatOpts))
    ).join(' ');
  }

  return interpolated;
}
