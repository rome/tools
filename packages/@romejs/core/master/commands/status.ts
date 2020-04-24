/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {WorkerStatus} from '../../common/bridges/WorkerBridge';
import {commandCategories} from '../../common/commands';
import {createMasterCommand} from '../commands';

type StatusResult = {
  master: {
    heapTotal: number;
    pid: number;
    uptime: number;
  };
  workers: Array<StatusWorkerResult>;
  projects: Array<{id: number}>;
};

type StatusWorkerResult = {
  astCacheSize: number;
  heapTotal: number;
  pid: number;
  uptime: number;
  ownedBytes: number;
  ownedFileCount: number;
};

export default createMasterCommand({
  category: commandCategories.PROCESS_MANAGEMENT,
  description: 'dump memory and process info of master and workers',
  usage: '',
  examples: [],

  defineFlags() {
    return {};
  },

  async callback({master}: MasterRequest): Promise<StatusResult> {
    const workers = await Promise.all(master.workerManager.getWorkers().map(
      async (worker): Promise<StatusWorkerResult> => {
        const workerStatus: WorkerStatus = await worker.bridge.status.call();

        return {
          astCacheSize: workerStatus.astCacheSize,
          heapTotal: workerStatus.memoryUsage.heapTotal,
          pid: workerStatus.pid,
          uptime: workerStatus.uptime,
          ownedBytes: worker.byteCount,
          ownedFileCount: worker.fileCount,
        };
      },
    ));

    const {heapTotal} = process.memoryUsage();
    return {
      master: {
        heapTotal,
        pid: process.pid,
        uptime: process.uptime(),
      },
      workers,
      projects: master.projectManager.getProjects().map((project) => {
        return {
          id: project.id,
        };
      }),
    };
  },
});
