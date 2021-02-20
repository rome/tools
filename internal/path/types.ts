import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import URLPath from "./classes/URLPath";

export type AnyFilePath = AbsoluteFilePath | RelativePath;

export type AnyPath = AbsoluteFilePath | RelativePath | URLPath | UIDPath;

export type PathSegments = string[];
