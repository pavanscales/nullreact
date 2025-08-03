// runtime/signals.js

export function signal(initial) {
  let value = initial;
  const subs = new Set();

  const read = () => {
    if (currentSubscriber) subs.add(currentSubscriber);
    return value;
  };

  const write = newValue => {
    if (newValue === value) return;
    value = newValue;
    for (const sub of subs) sub();
  };

  return [read, write];
}

let currentSubscriber = null;

/**
 * Tracks a reactive function.
 * @param {Function} fn - effect that uses signals
 */
export function effect(fn) {
  const wrapped = () => {
    cleanup(wrapped);
    currentSubscriber = wrapped;
    fn();
    currentSubscriber = null;
  };
  wrapped.deps = [];
  wrapped();
}

function cleanup(effectFn) {
  for (const dep of effectFn.deps || []) {
    dep.delete(effectFn);
  }
  effectFn.deps = [];
}
