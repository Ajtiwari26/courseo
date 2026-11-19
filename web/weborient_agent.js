/**
 * Courseo WebOrient Voice Co-Browsing & Ghost Cursor Client Adapter
 * Branded #FD6902 radiant tangerine cursor with smooth cubic-Bezier motion,
 * voice recognition (Web Speech API), text command fallback, and manifest routing.
 */
(function () {
  const scriptEl = document.currentScript;
  const config = {
    themeColor: (scriptEl && scriptEl.dataset.themeColor) || '#FD6902',
    manifestUrl: (scriptEl && scriptEl.dataset.manifest) || './weborient_manifest.json',
    agentName: (scriptEl && scriptEl.dataset.agentName) || 'Courseo Guide',
    greeting:
      (scriptEl && scriptEl.dataset.greeting) ||
      'Hi! I am your Courseo Guide. Ask me anything about our 12 courses or say "open classroom".'
  };

  let manifest = null;
  let cursorPos = { x: window.innerWidth - 120, y: window.innerHeight - 90 };
  let activeSpotlight = null;
  let recognition = null;
  let isListening = false;

  async function loadManifest() {
    try {
      const res = await fetch(config.manifestUrl);
      if (res.ok) manifest = await res.json();
    } catch (_) {
      manifest = null;
    }
  }

  function createGhostCursor() {
    const el = document.createElement('div');
    el.className = 'weborient-ghost-cursor';
    el.id = 'weborient-ghost-cursor';
    el.innerHTML = `
      <svg width="26" height="26" viewBox="0 0 24 24" fill="${config.themeColor}" stroke="#FFFFFF" stroke-width="1.5">
        <path d="M4 3l7.2 17.5 2.6-6.7 6.7-2.6L4 3z"/>
      </svg>
      <div class="weborient-cursor-tag">${config.agentName}</div>
    `;
    document.body.appendChild(el);
    return el;
  }

  function animateGhostCursorTo(cursorEl, targetX, targetY, duration = 680) {
    return new Promise((resolve) => {
      cursorEl.classList.add('visible');
      const startX = cursorPos.x;
      const startY = cursorPos.y;
      const ctrlX = (startX + targetX) / 2 + (targetY - startY) * 0.16;
      const ctrlY = Math.min(startY, targetY) - 48;
      const startTime = performance.now();

      function step(now) {
        const t = Math.min(1, (now - startTime) / duration);
        const ease = 1 - Math.pow(1 - t, 3);
        const x = (1 - ease) * (1 - ease) * startX + 2 * (1 - ease) * ease * ctrlX + ease * ease * targetX;
        const y = (1 - ease) * (1 - ease) * startY + 2 * (1 - ease) * ease * ctrlY + ease * ease * targetY;
        cursorEl.style.transform = `translate3d(${x}px, ${y}px, 0)`;
        if (t < 1) {
          requestAnimationFrame(step);
        } else {
          cursorPos = { x: targetX, y: targetY };
          resolve();
        }
      }
      requestAnimationFrame(step);
    });
  }

  async function spotlightSelector(cursorEl, selector, message, logFn) {
    const el = document.querySelector(selector);
    if (!el) return false;

    el.scrollIntoView({ behavior: 'smooth', block: 'center' });
    await new Promise((r) => setTimeout(r, 380));

    const rect = el.getBoundingClientRect();
    const targetX = Math.max(20, Math.min(window.innerWidth - 60, rect.left + rect.width * 0.45));
    const targetY = Math.max(20, Math.min(window.innerHeight - 60, rect.top + rect.height * 0.35));

    await animateGhostCursorTo(cursorEl, targetX, targetY);

    if (activeSpotlight) activeSpotlight.classList.remove('weborient-spotlight');
    el.classList.add('weborient-spotlight');
    activeSpotlight = el;

    if (logFn && message) logFn(message, 'agent');
    setTimeout(() => {
      if (activeSpotlight === el) el.classList.remove('weborient-spotlight');
    }, 4500);
    return true;
  }

  function buildWidget() {
    const cursorEl = createGhostCursor();
    const dock = document.createElement('div');
    dock.className = 'weborient-dock';
    dock.innerHTML = `
      <div class="weborient-panel" id="wo-panel" role="dialog" aria-label="${config.agentName}">
        <div style="display:flex; align-items:center; justify-content:space-between; padding:0.9rem 1rem; border-bottom:1px solid rgba(255,255,255,0.08);">
          <div style="display:flex; align-items:center; gap:0.55rem;">
            <span class="pulse-dot" style="background:${config.themeColor}; box-shadow:0 0 10px ${config.themeColor};"></span>
            <strong style="font-family:var(--font-display); font-size:0.92rem;">${config.agentName}</strong>
          </div>
          <button id="wo-close" type="button" style="background:transparent; border:none; color:#94A3B8; cursor:pointer; font-size:1rem;" aria-label="Close">×</button>
        </div>
        <div id="wo-feed" style="padding:0.95rem 1rem; max-height:220px; overflow-y:auto; display:flex; flex-direction:column; gap:0.65rem; font-size:0.84rem; line-height:1.5;">
          <div style="color:#CBD5E1;">${config.greeting}</div>
        </div>
        <div style="padding:0 1rem 0.75rem; display:flex; flex-wrap:wrap; gap:0.4rem;">
          <button class="ctrl-btn wo-quick" data-cmd="Spotlight OpenClaw & Hermes" type="button">OpenClaw Swarm</button>
          <button class="ctrl-btn wo-quick" data-cmd="Show SecureWatch DRM" type="button">SecureWatch</button>
          <button class="ctrl-btn wo-quick" data-cmd="Take me to Meta Ads" type="button">Meta Ads</button>
          <button class="ctrl-btn wo-quick" data-cmd="Open classroom" type="button">Classroom</button>
        </div>
        <form id="wo-form" style="display:flex; gap:0.45rem; padding:0.75rem 1rem; border-top:1px solid rgba(255,255,255,0.08); background:rgba(10,11,14,0.7);">
          <input id="wo-input" type="text" placeholder="Ask or command the Ghost Cursor..." style="flex:1; background:rgba(255,255,255,0.05); border:1px solid rgba(255,255,255,0.12); border-radius:999px; padding:0.5rem 0.85rem; font-size:0.82rem; color:#fff; outline:none;">
          <button id="wo-mic" type="button" class="ctrl-btn" title="Voice input" aria-label="Toggle voice input">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/></svg>
          </button>
        </form>
      </div>

      <button class="weborient-pill" id="wo-toggle" type="button" aria-expanded="false">
        <span class="wo-waveform" aria-hidden="true"><span></span><span></span><span></span><span></span></span>
        <span>Ask Courseo AI</span>
      </button>
    `;
    document.body.appendChild(dock);

    const panel = dock.querySelector('#wo-panel');
    const toggleBtn = dock.querySelector('#wo-toggle');
    const closeBtn = dock.querySelector('#wo-close');
    const feed = dock.querySelector('#wo-feed');
    const form = dock.querySelector('#wo-form');
    const input = dock.querySelector('#wo-input');
    const micBtn = dock.querySelector('#wo-mic');

    function appendMessage(text, role = 'agent') {
      const bubble = document.createElement('div');
      bubble.style.padding = '0.55rem 0.75rem';
      bubble.style.borderRadius = '10px';
      bubble.style.background = role === 'user' ? 'rgba(253,105,2,0.18)' : 'rgba(255,255,255,0.05)';
      bubble.style.color = role === 'user' ? '#FFFFFF' : '#CBD5E1';
      bubble.style.alignSelf = role === 'user' ? 'flex-end' : 'flex-start';
      bubble.textContent = text;
      feed.appendChild(bubble);
      feed.scrollTop = feed.scrollHeight;
    }

    async function handleCommand(raw) {
      const q = raw.trim().toLowerCase();
      if (!q) return;
      appendMessage(raw, 'user');

      if (q.includes('classroom') || q.includes('player') || q.includes('sandbox')) {
        const found = await spotlightSelector(cursorEl, '#tab-sandbox, #classroom-player-frame', 'Highlighting the interactive classroom sandbox.', appendMessage);
        if (!found) {
          appendMessage('Navigating to the Courseo Student Classroom...', 'agent');
          setTimeout(() => { window.location.href = './classroom.html'; }, 650);
        }
        return;
      }

      if (q.includes('openclaw') || q.includes('hermes') || q.includes('swarm') || q.includes('agent')) {
        const found = await spotlightSelector(
          cursorEl,
          "[data-course-id='autonomous-swarms-openclaw-hermes'], #detail-title",
          'Here is our flagship masterclass on Autonomous Multi-Agent Swarms with OpenClaw & Hermes.',
          appendMessage
        );
        if (!found) {
          appendMessage('Opening the OpenClaw & Hermes masterclass syllabus...', 'agent');
          setTimeout(() => { window.location.href = './course_detail.html?slug=autonomous-swarms-openclaw-hermes'; }, 650);
        }
        return;
      }

      if (q.includes('securewatch') || q.includes('drm') || q.includes('watermark')) {
        const found = await spotlightSelector(
          cursorEl,
          '#hero-securewatch, #classroom-player-frame, #bento-securewatch',
          'Courseo SecureWatch protects streams with 60Hz anti-camcording shaders and BCH(64,48) forensic watermarking.',
          appendMessage
        );
        if (!found) {
          appendMessage('Taking you to the live SecureWatch showcase...', 'agent');
          setTimeout(() => { window.location.href = './index.html#hero-securewatch'; }, 650);
        }
        return;
      }

      if (q.includes('meta') || q.includes('ads') || q.includes('shopify') || q.includes('growth')) {
        const found = await spotlightSelector(
          cursorEl,
          "[data-course-id='meta-ads-2026-scaling']",
          'Here is Meta Ads 2026/2027 Scaling in our Growth & Commerce track.',
          appendMessage
        );
        if (!found) {
          appendMessage('Opening Growth & Commerce courses in the catalog...', 'agent');
          setTimeout(() => { window.location.href = './courses.html?track=growth-commerce'; }, 650);
        }
        return;
      }

      if (q.includes('film') || q.includes('cinema') || q.includes('seedance') || q.includes('higgsfield') || q.includes('music')) {
        appendMessage('Opening Generative AI Filmmaking & Creative Media courses...', 'agent');
        setTimeout(() => { window.location.href = './courses.html?track=generative-media'; }, 650);
        return;
      }

      if (q.includes('catalog') || q.includes('all courses') || q.includes('browse')) {
        appendMessage('Opening the complete 12-course catalog...', 'agent');
        setTimeout(() => { window.location.href = './courses.html'; }, 600);
        return;
      }

      const highlighted = await spotlightSelector(
        cursorEl,
        '#curriculum, #catalog-grid, #syllabus',
        'I can spotlight courses, filter tracks (AI Engineering, Generative Media, Growth), or open the Classroom.',
        appendMessage
      );
      if (!highlighted) {
        appendMessage('Try saying "Spotlight OpenClaw", "Show SecureWatch", "Meta Ads", or "Open classroom".', 'agent');
      }
    }

    toggleBtn.addEventListener('click', () => {
      const open = panel.classList.toggle('open');
      toggleBtn.setAttribute('aria-expanded', String(open));
      if (open) input.focus();
    });

    closeBtn.addEventListener('click', () => {
      panel.classList.remove('open');
      toggleBtn.setAttribute('aria-expanded', 'false');
    });

    form.addEventListener('submit', (e) => {
      e.preventDefault();
      const value = input.value;
      input.value = '';
      handleCommand(value);
    });

    dock.querySelectorAll('.wo-quick').forEach((btn) => {
      btn.addEventListener('click', () => handleCommand(btn.dataset.cmd));
    });

    // Optional Web Speech API voice input
    const SpeechRec = window.SpeechRecognition || window.webkitSpeechRecognition;
    if (SpeechRec) {
      recognition = new SpeechRec();
      recognition.lang = 'en-US';
      recognition.interimResults = false;
      recognition.onresult = (event) => {
        const transcript = event.results[0][0].transcript;
        handleCommand(transcript);
      };
      recognition.onend = () => {
        isListening = false;
        micBtn.classList.remove('active');
      };
    }

    micBtn.addEventListener('click', () => {
      if (!recognition) {
        appendMessage('Voice recognition is unavailable in this browser—type your request below.', 'agent');
        return;
      }
      if (isListening) {
        recognition.stop();
      } else {
        isListening = true;
        micBtn.classList.add('active');
        recognition.start();
      }
    });

    // Expose programmatic API for page buttons
    window.CourseoWebOrient = {
      spotlight: (selector, message) => {
        panel.classList.add('open');
        return spotlightSelector(cursorEl, selector, message, appendMessage);
      },
      command: (text) => {
        panel.classList.add('open');
        return handleCommand(text);
      },
      getManifest: () => manifest
    };
  }

  loadManifest().finally(buildWidget);
})();