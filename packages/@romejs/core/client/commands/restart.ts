/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from '../../common/commands';
import {createLocalCommand} from '../commands';
import ClientRequest from '../ClientRequest';

export default createLocalCommand({
  category: commandCategories.PROCESS_MANAGEMENT,
  description: 'restart daemon',
  usage: '',
  examples: [],

  defineFlags() {
    return {};
  },

  async callback(req: ClientRequest) {
    const stopped = await req.client.query({
      command: 'stop',
    });

    if (stopped.type === 'SUCCESS' && stopped.data === true) {
      const started = await req.client.query({
        command: 'start',
      });
      return started.type === 'SUCCESS' && started.data === true;
    } else {
      return false;
    }
  },
});
