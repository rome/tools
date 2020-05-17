export async function fun1Async(ex) {
  function delay(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  if (ex) {
    await delay(300);
    throw new Error(ex);
  } else {
    return 'Hello world';
  }
}

export function fun1(ex) {
  if (ex) {
    throw new Error(ex);
  } else {
    return 'Hello world';
  }
}
