import { AsyncVoidCallback } from "@internal/typescript-helpers";
import Resource from "./Resource";
import { ResourceOptions, ResourcesContainer } from "./types";

export type {Resource};

export function extractResource(rawResource: ResourcesContainer | Resource): Resource {
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

export function createResourceFromCallback(name: string, callback: AsyncVoidCallback, opts?: Omit<ResourceOptions, "release">): Resource {
  return new Resource(name, {
    ...opts,
    release: callback,
  });
}

export function createResourceRoot(name: string): Resource {
  const resource = new Resource(name);
  processResourceRoot.add(resource);
  return resource;
}

export function createResource(name: string, opts?: Omit<ResourceOptions, "release">): Resource {
  return new Resource(name, opts);
}

export const processResourceRoot = new Resource("Process", {optional: true});

// Only called when event loop has ran out of work, not when fatal signals are caught
process.on("beforeExit", () => {
  safeProcessExit();
});

export async function safeProcessExit(code: number = 0): Promise<never> {
  await processResourceRoot.release();
  process.exit(code);
}