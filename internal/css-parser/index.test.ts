import {declareParserTests} from "@internal/test-helpers";

const promise = declareParserTests();

// @ts-expect-error Doesn't support top-level await lol
await promise;
