/* appearance.js — style + theme controller for the on-page "Aa" menu
 * (from the claude-design resume project).
 * The page's data-aesthetic is the default style (`resume generate --style`).
 * The theme is data-theme-default when set (`--theme light|dark`), otherwise it
 * follows the OS (prefers-color-scheme). A visitor's pick wins over both and is
 * saved in localStorage. */
(function () {
  var root = document.documentElement;
  var KEY = 'resume-tweaks';
  var mq = window.matchMedia ? window.matchMedia('(prefers-color-scheme: dark)') : null;
  var saved = {};
  try { saved = JSON.parse(localStorage.getItem(KEY) || '{}') || {}; } catch (e) {}
  var defaultTheme = root.getAttribute('data-theme-default');

  var state = {
    aesthetic: saved.aesthetic || root.getAttribute('data-aesthetic') || 'editorial',
    theme: saved.theme || defaultTheme || (mq && mq.matches ? 'dark' : 'light')
  };
  var userTheme = !!saved.theme;
  var subs = new Set();

  function apply() {
    root.setAttribute('data-aesthetic', state.aesthetic);
    root.setAttribute('data-theme', state.theme);
  }
  function save() {
    var out = { aesthetic: state.aesthetic };
    if (userTheme) out.theme = state.theme;
    try { localStorage.setItem(KEY, JSON.stringify(out)); } catch (e) {}
  }
  function notify() { subs.forEach(function (fn) { fn(Object.assign({}, state)); }); }

  window.Appearance = {
    get: function () { return Object.assign({}, state); },
    set: function (patch) {
      if (patch.theme) userTheme = true;
      state = Object.assign({}, state, patch);
      apply(); save(); notify();
    },
    subscribe: function (fn) { subs.add(fn); return function () { subs.delete(fn); }; }
  };

  if (mq) {
    var onSys = function (e) {
      if (userTheme || defaultTheme) return;
      state.theme = e.matches ? 'dark' : 'light';
      apply(); notify();
    };
    if (mq.addEventListener) mq.addEventListener('change', onSys); else if (mq.addListener) mq.addListener(onSys);
  }

  apply();

  function wireBar() {
    var bar = document.getElementById('appbar');
    if (!bar) return;
    var A = window.Appearance;
    var toggle = document.getElementById('appbar-toggle');
    var pop = document.getElementById('appbar-pop');

    function open() {
      pop.hidden = false;
      toggle.setAttribute('aria-expanded', 'true');
      document.addEventListener('pointerdown', onOutside, true);
      document.addEventListener('keydown', onKey);
    }
    function close() {
      pop.hidden = true;
      toggle.setAttribute('aria-expanded', 'false');
      document.removeEventListener('pointerdown', onOutside, true);
      document.removeEventListener('keydown', onKey);
    }
    function onOutside(e) { if (!bar.contains(e.target)) close(); }
    function onKey(e) { if (e.key === 'Escape') { close(); toggle.focus(); } }

    toggle.addEventListener('click', function () { if (pop.hidden) open(); else close(); });
    bar.querySelectorAll('[data-style]').forEach(function (b) {
      b.addEventListener('click', function () { A.set({ aesthetic: b.dataset.style }); });
    });
    bar.querySelectorAll('[data-theme-set]').forEach(function (b) {
      b.addEventListener('click', function () { A.set({ theme: b.dataset.themeSet }); });
    });

    function reflect(s) {
      bar.querySelectorAll('[data-style]').forEach(function (b) {
        b.setAttribute('aria-checked', String(b.dataset.style === s.aesthetic));
      });
      bar.querySelectorAll('[data-theme-set]').forEach(function (b) {
        b.setAttribute('aria-checked', String(b.dataset.themeSet === s.theme));
      });
    }
    A.subscribe(reflect);
    reflect(A.get());
  }

  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', wireBar);
  else wireBar();
})();
