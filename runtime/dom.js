import { effect } from './signals.js';

// TEXT NODE POOL TO REUSE NODES
const pool = [];

/**
 * Insert reactive or static content.
 */
export function insert(parent, value) {
  if (typeof value === 'function') {
    const node = pool.pop() || document.createTextNode('');
    parent.appendChild(node);
    let prev = null;

    effect(() => {
      const next = value();
      if (next !== prev) {
        prev = next;
        node.textContent = next == null ? '' : next;
      }
    });
  } else {
    parent.appendChild(document.createTextNode(value ?? ''));
  }
}

/**
 * Set attribute reactively.
 */
export function setAttr(el, name, value) {
  if (typeof value === 'function') {
    let prev = null;
    effect(() => {
      const next = value();
      if (next !== prev) {
        prev = next;
        el.setAttribute(name, next);
      }
    });
  } else {
    el.setAttribute(name, value);
  }
}

/**
 * Set event listeners.
 */
export function on(el, event, handler) {
  el.addEventListener(event, handler);
}
