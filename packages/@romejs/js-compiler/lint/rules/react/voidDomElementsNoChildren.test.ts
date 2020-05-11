/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
  'prevent void element from using children or dangerouslySetInnerHTML',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        "<br className={'dont-remove-class'}>invalid children</br>",
        '<img>invalid children</img>',
        "<hr children={'invalid children'} />",
        "<area dangerouslySetInnerHTML={{__html: ''}}></area>",
        "<img dangerouslySetInnerHTML={{__html: ''}} children={'invalid children'}>invalid children</img>",
        // VALID
        '<div>Children</div>',
        "<div children='Children' />",
        "<div dangerouslySetInnerHTML={{ __html: '' }} />",
        "<br className='valid' />",
      ],
      {category: 'lint/voidDomElementsNoChildren'},
    );
  },
);
