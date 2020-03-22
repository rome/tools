/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Bridge} from '@romejs/events';
import {WebMasterRequest, WebMasterClient} from '../../master/web';

export default class WebBridge extends Bridge {
  requests = this.createEvent<{
    requests: Array<WebMasterRequest>;
    clients: Array<WebMasterClient>;
  }, void>({
    name: 'WebBridge.requests',
    direction: 'server->client',
  });
}
