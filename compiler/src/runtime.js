// ultra fast reactive runtime for web - Linux bare-metal mindset

let currentEffect = null;
const effectStack = [];
const pendingEffects = [];
let scheduled = false;

export function effect(fn) {
  const wrapped = () => {
    cleanup(wrapped);
    currentEffect = wrapped;
    effectStack.push(wrapped);
    fn();
    effectStack.pop();
    currentEffect = effectStack[effectStack.length - 1] || null;
  };
  wrapped.deps = [];
  wrapped();
  return wrapped;
}

function cleanup(effectFn) {
  const deps = effectFn.deps;
  for (let i = 0; i < deps.length; i++) {
    deps[i].delete(effectFn);
  }
  effectFn.deps.length = 0;
}

export function signal(initialValue) {
  let value = initialValue;
  const subscribers = new Set();

  return {
    get() {
      if (currentEffect) {
        if (!subscribers.has(currentEffect)) {
          subscribers.add(currentEffect);
          currentEffect.deps.push(subscribers);
        }
      }
      return value;
    },
    set(newValue) {
      if (value !== newValue) {
        value = newValue;
        // push subscribers to batch queue (no duplicates)
        for (const eff of subscribers) scheduleEffect(eff);
      }
    }
  };
}

function scheduleEffect(eff) {
  // avoid duplicates
  if (!pendingEffects.includes(eff)) {
    pendingEffects.push(eff);
  }
  if (!scheduled) {
    scheduled = true;
    Promise.resolve().then(runEffects);
  }
}

function runEffects() {
  scheduled = false;
  const effects = pendingEffects.splice(0, pendingEffects.length);
  // fast for loop, no closures
  for (let i = 0; i < effects.length; i++) {
    effects[i]();
  }
}

// DOM helpers - direct, zero overhead

export function createElement(tag) {
  return document.createElement(tag);
}

export function setAttribute(el, key, value) {
  if (value === false || value == null) el.removeAttribute(key);
  else el.setAttribute(key, value === true ? '' : value);
}

export function insert(parent, child, anchor = null) {
  parent.insertBefore(child, anchor);
}

export function remove(node) {
  if (node.parentNode) node.parentNode.removeChild(node);
}

export function textNode(content) {
  return document.createTextNode(content);
}

export function setText(node, content) {
  if (node.textContent !== content) node.textContent = content;
}

// Mount JSX-like AST node to DOM with zero overhead reactive text updates

export function mount(node, container) {
  if (node.type === 'text') {
    const t = textNode(node.content);
    insert(container, t);
    return t;
  }
  if (node.type === 'element') {
    const el = createElement(node.tag);
    const attrs = node.attrs;
    if (attrs) {
      for (const key in attrs) {
        setAttribute(el, key, attrs[key]);
      }
    }
    const children = node.children;
    if (children) {
      for (let i = 0; i < children.length; i++) {
        mount(children[i], el);
      }
    }
    insert(container, el);
    return el;
  }
  if (node.type === 'signal-text') {
    const t = textNode('');
    insert(container, t);
    effect(() => {
      setText(t, node.get());
    });
    return t;
  }
}
