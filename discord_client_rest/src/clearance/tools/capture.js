copy(
  (function () {
    const out = {};
    const cls = (v) => {
      if (v === null) return 'x';
      switch (typeof v) {
        case 'undefined': return 'u';
        case 'boolean': return v ? 'T' : 'F';
        case 'function': return 'N';
        case 'number': return String(v);
        case 'string': return v;
      }
      return Array.isArray(v) ? v.join(',') : 'o';
    };
    const scan = (obj, prefix) => {
      const names = new Set();
      for (const k in obj) names.add(k);
      for (let o = obj; o && o !== Object.prototype; o = Object.getPrototypeOf(o))
        for (const k of Object.getOwnPropertyNames(o)) names.add(k);
      for (const k of names) {
        try { out[prefix + k] = cls(obj[k]); } catch (e) {}
      }
    };
    scan(window, '');
    scan(navigator, 'n.');
    scan(document, 'd.');
    return JSON.stringify(out);
  })()
);
