/* should not generate diagnostics */

function a() { }
export { a };

export function b() { }

export const { A } = { A: 1 };
export const [B] = [1];

export declare const valid;
