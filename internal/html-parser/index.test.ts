import {declareParserTests} from "@internal/test-helpers";

const promise = declareParserTests();

// @ts-ignore Doesn't support top-level await lol
await promise;
