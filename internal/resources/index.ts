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
  return new Resource(name, {
    optional: true,
  });
}

export function createResource(name: string, opts?: Omit<ResourceOptions, "release">): Resource {
  return new Resource(name, opts);
}