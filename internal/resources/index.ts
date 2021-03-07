import {IS_ROME_DEV_ENV} from "@internal/cli-environment";
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
    // Suppress all uncaught exceptions. We'll begin tearing down and don't want Node's error handlers to be used
    process.setUncaughtExceptionCaptureCallback(null);
    process.setUncaughtExceptionCaptureCallback((err) => {
      console.error(err);
    });
    process.on("unhandledRejection", () => {});

    async function registerError() {
      await drainProcessStreams();

      if (process.exitCode === 0) {
        // Not successful anymore
        process.exitCode = 2;
      }
    }

    try {
      await drainProcessStreams();
      
      // If we are in development mode, then verify we cleaned up properly
      if (IS_ROME_DEV_ENV) {
        // @ts-ignore: Internal node methods that we are being naughty and touching
        const handles = new Set(process._getActiveHandles());
        
        // @ts-ignore: Internal node methods that we are being naughty and touching
        const requests = process._getActiveRequests();

        // Remove io streams as they are the only valid handle that should exist
        handles.delete(process.stdout);
        handles.delete(process.stderr);
        handles.delete(process.stdin);
        handles.delete(workerThreads.parentPort);
        if (handles.size > 0) {
          console.error(`safeProcessExit: Handles found after teardown`)
          console.error(handles);
          await registerError();
        }
        
        if (requests.length > 0) {
          console.error(`safeProcessExit: Pending requests found`);
          console.error(requests);
          await registerError();
        }
      }
    } catch (err) {
      console.error("safeProcessExit: Error occurred during process teardown:");
      console.error(err.stack);
      await registerError();
    } finally {
      process.exit(process.exitCode);
    }
  },
});

function createFlushPromise(stream: NodeJS.WriteStream): Promise<void> {
  return new Promise((resolve) => {
    stream.write("", () => {
      resolve();
    });
  });
}

async function drainProcessStreams(): Promise<void> {
  await Promise.all([
    createFlushPromise(process.stdout),
    createFlushPromise(process.stderr),
  ]);
}

async function drainExit(code: number): Promise<void> {
  await drainProcessStreams();
  process.exit(code);
}

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

  try {
    await processResourceRoot.release();
  } catch (err) {
    console.error(`safeProcessExit: Release failure`);
    console.error(err.stack);
    await drainExit(2);
  }

  await drainExit(2);
}
