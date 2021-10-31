export async function *getGenerator<T>(
  duration: number,
  func: (...args: any[]) => Promise<T>,
  ...args
): AsyncIterableIterator<T> {

  let timeout = async () => new Promise(
    resolve => setTimeout(resolve, duration)
  );

  while(true) {
    yield await func(...args);
    await timeout();
  }
}