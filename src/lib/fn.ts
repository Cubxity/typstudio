export type ThrottleFn<T extends never[]> = (...args: T) => PromiseLike<void>;

export const throttle = <T extends never[]>(
  fn: (...args: T) => PromiseLike<never>
): ThrottleFn<T> => {
  let isExecuting = false;
  let pendingArgs: T | undefined;

  const executeGuard = async (...args: T) => {
    isExecuting = true;

    // Remove pending call before executing
    pendingArgs = undefined;
    try {
      await fn(...args);
    } finally {
      isExecuting = false;
    }
  };

  return async (...args) => {
    if (!isExecuting) {
      let executeArgs: T | undefined = args;
      // Nothing is executing, immediately execute the function
      while (executeArgs) {
        await executeGuard(...executeArgs);
        // Execute the pending call, if exists.
        executeArgs = pendingArgs;
      }
      return;
    }

    // Something is currently executing, queue the args and set a timeout
    pendingArgs = args;
  };
};
