// ultra fast reactive runtime - kernel-tier optimized

let curEff = null;
const stack = [];
const pending = new Set();
let scheduled = false;

// Reactive Signal
export function signal(val) {
  const subs = new Set();
  const s = {
    get() {
      if (curEff) {
        subs.add(curEff);
        curEff.deps.push(subs);
      }
      return val;
    },
    set(newVal) {
      if (val !== newVal) {
        val = newVal;
        for (const eff of subs) schedule(eff);
      }
    }
  };
  return s;
}

// Reactive Effect
export function effect(fn) {
  const eff = () => {
    cleanup(eff);
    curEff = eff;
    stack.push(eff);
    fn();
    stack.pop();
    curEff = stack[stack.length - 1] || null;
  };
  eff.deps = [];
  eff();
  return eff;
}

function cleanup(eff) {
  const d = eff.deps;
  for (let i = 0; i < d.length; i++) d[i].delete(eff);
  d.length = 0;
}

function schedule(eff) {
  if (!pending.has(eff)) pending.add(eff);
  if (!scheduled) {
    scheduled = true;
    queueMicrotask(flush);
  }
}

function flush() {
  scheduled = false;
  const q = Array.from(pending);
  pending.clear();
  for (let i = 0; i < q.length; i++) q[i]();
}

// Pure DOM ops - raw and unabstracted

export const el = tag => document.createElement(tag);
export const txt = str => document.createTextNode(str);
export const ins = (p, c, a = null) => p.insertBefore(c, a);
export const rm = n => n.parentNode?.removeChild(n);
export const attr = (el, k, v) =>
  v == null || v === false
    ? el.removeAttribute(k)
    : el.setAttribute(k, v === true ? '' : v);
export const setTxt = (n, c) => {
  if (n.textContent !== c) n.textContent = c;
};

// JSX-style AST mount
export function mount(n, container) {
  if (n.type === 'text') {
    const t = txt(n.content);
    ins(container, t);
    return t;
  }

  if (n.type === 'signal-text') {
    const t = txt('');
    ins(container, t);
    effect(() => setTxt(t, n.get()));
    return t;
  }

  if (n.type === 'element') {
    const e = el(n.tag);
    const a = n.attrs;
    if (a) for (const k in a) attr(e, k, a[k]);
    const ch = n.children;
    if (ch) for (let i = 0; i < ch.length; i++) mount(ch[i], e);
    ins(container, e);
    return e;
  }
}
