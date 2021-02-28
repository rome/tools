import { AsyncVoidCallback } from "@internal/typescript-helpers";
import {enhanceNodeInspectClass} from "@internal/node";
import {ResourceOptions, ResourceTree, ResourcesContainer} from "./types";
import {createResourceFromCallback, extractResource} from "./index";
import workerThreads = require("worker_threads");
import childProcess = require("child_process");
import net = require("net");
import { getEnvVar } from "@internal/cli-environment";

const isDebugStack = getEnvVar("ROME_RESOURCE_STACKS").type === "ENABLED";

export default class Resource {
  constructor(name: string, opts: ResourceOptions = {}) {
    this.options = opts;
    this.closed = false;
    this.resources = new Set();
    this.owners = new Set();
    this[Symbol.toStringTag] = `Resource<${name}>`;

    // If this resource isn't optional then it must be attached to something
    if (!opts.optional) {
      let timeoutError: undefined | Error;
      let timeoutMessage = `The resource ${name} is not correctly managed as it has not been attached to a parent resource`;
      if (isDebugStack) {
        timeoutError = new Error(timeoutMessage);
      }

      this.timeout = setTimeout(() => {
        if (timeoutError === undefined) {
          throw new Error(`${timeoutMessage}. Set the environment variable ROME_RESOURCE_STACKS=1 for a creation stack.`);
        } else {
          throw timeoutError;
        }
      }, 0);
    }
  }

  public [Symbol.toStringTag]: string;

  protected timeout: undefined | NodeJS.Timeout;
  private options: ResourceOptions;
  private closed: boolean;
  private owners: Set<Resource>;
  private resources: Set<Resource>;

  private markAttached(): void {
    if (this.timeout !== undefined) {
      clearTimeout(this.timeout);
      this.timeout = undefined;
    }
  }

  public buildTree(): ResourceTree {
    const tree: ResourceTree = new Map();
    tree.set(this, this.buildSubTree());
    return tree;
  }

  public buildSubTree(seen: Set<Resource> = new Set()): ResourceTree {
    const tree: ResourceTree = new Map();
    for (const resource of this.resources) {
      if (seen.has(resource)) {
        // Ideally we should actually error? Resources are a tree, not a graph
        continue;
      }

      tree.set(resource, resource.buildSubTree(seen));
    }
    return tree;
  }

  public release(): Promise<void> {
    return this._release(this);
  }

  private async _release(root: Resource): Promise<void> {
    if (this.closed) {
      return;
    }

    this.closed = true;
    this.markAttached();

    // Release all sub resources
    const promises = Array.from(this.resources, (resource) => {
      return resource._release(root);
    });

    // Release ourselves
    const {release} = this.options;
    if (release !== undefined) {
      const selfPromise: Promise<void> = new Promise((resolve) => {
        resolve(release());
      });

      const timeout = setTimeout(() => {
        // TODO reject?
        console.log(`${this[Symbol.toStringTag]} has not been released after 3 seconds`);
      }, 3000);

      promises.push(selfPromise.finally(() => {
        clearTimeout(timeout);
      }));
    }
    
    await Promise.allSettled(promises);

    // Remove ourselves from resources we've been added as a dependency to
    for (const owner of this.owners) {
      owner.remove(this);
    }
    this.owners.clear();
  }

  public create(name: string, opts?: ResourceOptions): Resource {
    const resource = new Resource(name, opts);
    this.add(resource);
    return resource;
  }

  public add(rawResource: ResourcesContainer | Resource, markAttachment: boolean = true): Resource {
    const resource = extractResource(rawResource);

    if (resource.closed || this.closed || this.resources.has(resource)) {
      return resource;
    }

    if (markAttachment) {
      resource.markAttached();
    }

    this.resources.add(resource);
    resource.owners.add(this);

    return resource;
  }

  public remove(rawResource: ResourcesContainer | Resource): void {
    this.resources.delete(extractResource(rawResource));
  }

  public addCallback(name: string, callback: AsyncVoidCallback): Resource {
    return this.add(createResourceFromCallback(name, callback));
  }

  public addTimeout(name: string, timeout: NodeJS.Timeout): Resource {
    return this.addCallback(`setTimeout<${name}>`, () => {
      clearTimeout(timeout);
    });
  }

  public addChildProcess(proc: childProcess.ChildProcess): Resource {
    const resource = this.create("child_process.ChildProcess", {
      release: () => {
        proc.kill();
      },
    });
    proc.on("close", () => {
      resource.release();
    });
    return resource;
  }

  public addWorkerThread(worker: workerThreads.Worker): Resource {
    const resource = this.create("worker_threads.Worker", {
      release: async () => {
        await worker.terminate();
      },
    });
    worker.on("exit", () => {
      resource.release();
    });
    return resource;
  }

  public addSocket(socket: net.Socket): Resource {
    const resource = this.create("net.Socket", {
      release: () => {
        socket.end();
      },
    });
    socket.on("close", () => {
      resource.release();
    });
    return resource;
  }

  public addServer(server: net.Server): Resource {
    const resource = this.create("net.Server", {
      release: () => {
        // NB: Should we use the callback arg?
        server.close();
      },
    });
    server.on("close", () => {
      resource.release();
    });
    return resource;
  }

  public addWebSocket(socket: WebSocket): Resource {
    const resource = this.create("WebSocket", {
      release: () => {
        socket.close();
      },
    });
    socket.addEventListener("open", () => {
      resource.release();
    });
    return resource;
  }
}

// @ts-ignore: Yes I really want to do this on an abstract class... bite me
enhanceNodeInspectClass(Resource, (resc) => {
  return resc[Symbol.toStringTag];
});