// Used when we have logged an error to the client, but want to stop execution
// This will cause the error stack to not be printed but will still result in an error exit
export default class SilentClientError extends Error {}
