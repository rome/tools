/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../../api/lint.test';

test(
  'no explicit any',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // VALID
        'const age: number = 17;age;',
        'const ages: Array<number> = [17];ages;',
        'function greet(): string {};greet();',
        'function greet(): Array<string> {};greet();',
        'function greet(): Array<Array<string>> {};greet();',
        'function greet(param: Array<string>): string { return param; };greet();',
        'function greet(param: Array<string>): Array<string> { return param; };greet();',

        // INVALID
        "const age: any = 'seventeen';age;",
        "const ages: any[] = ['seventeen'];ages;",
        "const ages: Array<any> = ['seventeen'];ages;",
        'function greet(): any {};greet();',
        'function greet(): any[] {};greet();',
        'function greet(): Array<any> {};greet();',
        'function greet(): Array<Array<any>> {};greet();',
        'function greet(param: Array<any>): string { return param; };greet();',
        'function greet(param: Array<any>): Array<any> { return param; };greet();',
      ],
      {category: 'lint/noExplicitAny', syntax: ['ts']},
    );
  },
);
