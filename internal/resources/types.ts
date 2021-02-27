import { AsyncVoidCallback } from "@internal/typescript-helpers";
import Resource from "./Resource";

export type ResourceOptions = {
  optional?: boolean;
  release?: AsyncVoidCallback;
};

export type ResourcesContainer = {
  resources: undefined | Resource;
};

export type ResourceTree = Map<Resource, ResourceTree>;