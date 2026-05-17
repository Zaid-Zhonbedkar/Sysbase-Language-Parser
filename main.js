// ── NAV SCROLL ──
const nav = document.getElementById('nav');
window.addEventListener('scroll', () => {
  nav.classList.toggle('scrolled', window.scrollY > 20);
});

// ── HAMBURGER ──
const ham = document.getElementById('hamburger');
const mob = document.getElementById('mobileMenu');
ham.addEventListener('click', () => mob.classList.toggle('open'));
document.querySelectorAll('.mobile-menu a').forEach(a => a.addEventListener('click', () => mob.classList.remove('open')));

// ── CODE TABS ──
function switchCode(lang, btn) {
  document.querySelectorAll('.ctab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.code-panel').forEach(p => p.classList.remove('active'));
  btn.classList.add('active');
  document.getElementById('panel-' + lang).classList.add('active');
}

// ── COPY INSTALL ──
function copyCmd() {
  navigator.clipboard.writeText('cargo add sysbase').then(() => {
    const lbl = document.getElementById('copyLabel');
    lbl.textContent = 'copied!';
    lbl.style.color = 'var(--green)';
    setTimeout(() => { lbl.textContent = 'copy'; lbl.style.color = ''; }, 2000);
  });
}

// ── SCROLL REVEAL ──
const revealObs = new IntersectionObserver((entries) => {
  entries.forEach(e => {
    if (e.isIntersecting) {
      e.target.classList.add('visible');
      // Animate perf bars
      e.target.querySelectorAll('.bar-fill').forEach(b => {
        const w = b.getAttribute('data-w');
        if (w) setTimeout(() => b.style.width = w + '%', 100);
      });
      e.target.querySelectorAll('.mem-fill').forEach(b => {
        const w = b.getAttribute('data-w');
        if (w) setTimeout(() => b.style.width = w + '%', 100);
      });
      // Animate counters
      e.target.querySelectorAll('.counter').forEach(c => {
        const target = parseInt(c.getAttribute('data-target'), 10);
        if (target === 0) { c.textContent = '0'; return; }
        const dur = 1800, step = target / (dur / 16);
        let cur = 0;
        const tick = () => {
          cur = Math.min(cur + step, target);
          c.textContent = Math.floor(cur);
          if (cur < target) requestAnimationFrame(tick);
        };
        tick();
      });
      revealObs.unobserve(e.target);
    }
  });
}, { threshold: 0.1, rootMargin: '0px 0px -40px 0px' });

document.querySelectorAll('.reveal,.reveal-l,.reveal-r').forEach(el => revealObs.observe(el));

// ── STAGGER delays on feat cards ──
document.querySelectorAll('.feat-card').forEach((c, i) => {
  c.style.transitionDelay = (i * 0.05) + 's';
});

// ── SMOOTH ANCHOR SCROLL ──
document.querySelectorAll('a[href^="#"]').forEach(a => {
  a.addEventListener('click', e => {
    const id = a.getAttribute('href');
    if (id === '#') return;
    const el = document.querySelector(id);
    if (el) { e.preventDefault(); el.scrollIntoView({ behavior: 'smooth', block: 'start' }); }
  });
});
function scrollTo(id) {
  const el = document.querySelector(id);
  if (el) el.scrollIntoView({ behavior: 'smooth' });
}
