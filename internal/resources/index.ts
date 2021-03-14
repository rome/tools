import { createResourceFromCallback } from "./factories";
import Resource from "./Resource";
import {ResourcesContainer} from "./types";
import workerThreads = require("worker_threads");

export type {Resource};

export * from "./factories";

export function extractResource(rawResource: ResourcesContainer): Resource {
  if (rawResource instanceof Resource) {
    return rawResource;
  } else {
    const prop = rawResource.resources;
    if (prop instanceof Resource) {
      return prop;
    } else {
      throw new Error(`Resource not provided`);
    }
  }
}

export const processResourceRoot = new Resource({
  name: "Process",
  getDetails: () => ({
    type: "process",
    self: true,
    pid: process.pid,
    tid: workerThreads.threadId,
    command: process.execPath,
    args: process.argv.slice(1),
  }),
  optional: true,
  async finalize() {
    process.exit(process.exitCode);
  },
});

const processResourceRelease = processResourceRoot.release.bind(processResourceRoot);

// Only ran when the event loop has ran out of work, we cannot "catch" exits si we need use safeProcessExit calls
process.on("beforeExit", processResourceRelease);
process.on("SIGINT", processResourceRelease);
process.on("SIGHUP", processResourceRelease);
process.on("SIGTERM", processResourceRelease);

// We explicitly do not attach this to processResourceRelease as those should always be attached during teardown
export const processResourceListeners = createResourceFromCallback("ProcessListeners", () => {
  process.removeListener("beforeExit", processResourceRelease);
  process.removeListener("SIGINT", processResourceRelease);
  process.removeListener("SIGHUP", processResourceRelease);
  process.removeListener("SIGTERM", processResourceRelease);
}, {
  optional: true,
});

export async function safeProcessExit(code: number): Promise<void> {
  process.exitCode = code;

  // Suppress all uncaught exceptions. We'll begin tearing down and don't want Node's error handlers to be used
  process.setUncaughtExceptionCaptureCallback(null);
  process.setUncaughtExceptionCaptureCallback(() => {});
  process.removeAllListeners("unhandledRejection");
  process.on("unhandledRejection", () => {});

  try {
    await processResourceRoot.releaseBlock();
  } catch (err) {
    console.error(`safeProcessExit: Release failure`);
    console.error(err.stack);
  }

  console.error(`safeProcessExit: Process not exited by resource release`);
  process.exit(2);
}
