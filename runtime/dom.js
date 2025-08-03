import { effect, onCleanup } from './signals.js';

// Text node reuse pool
const pool = [];

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
  } else if (Array.isArray(value)) {
    insertArray(parent, value);
  } else if (value instanceof Node) {
    parent.appendChild(value);
  } else {
    parent.appendChild(document.createTextNode(value ?? ''));
  }
}

function insertArray(parent, items) {
  const nodes = [];

  for (let item of items) {
    if (typeof item === 'function') {
      const node = document.createTextNode('');
      parent.appendChild(node);
      nodes.push(node);
      let prev = null;

      effect(() => {
        const next = item();
        if (next !== prev) {
          prev = next;
          node.textContent = next == null ? '' : next;
        }
      });
    } else if (item instanceof Node) {
      parent.appendChild(item);
      nodes.push(item);
    } else {
      const node = document.createTextNode(item ?? '');
      parent.appendChild(node);
      nodes.push(node);
    }
  }

  onCleanup(() => {
    for (const node of nodes) node.remove();
  });
}

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

export function on(el, event, handler) {
  el.addEventListener(event, handler);
  onCleanup(() => el.removeEventListener(event, handler));
}

export function insertHTML(parent, html) {
  parent.innerHTML = html;
}

export function clear(parent) {
  while (parent.firstChild) {
    parent.removeChild(parent.firstChild);
  }
}

// 🔥 JSX helper: auto wraps elements with props and children
export function createElement(tag, props, ...children) {
  const el = typeof tag === 'function' ? tag(props || {}) : document.createElement(tag);

  if (props) {
    for (let [key, value] of Object.entries(props)) {
      if (key.startsWith('on')) {
        on(el, key.slice(2).toLowerCase(), value);
      } else {
        setAttr(el, key, value);
      }
    }
  }

  for (let child of children) {
    insert(el, child);
  }

  return el;
}
