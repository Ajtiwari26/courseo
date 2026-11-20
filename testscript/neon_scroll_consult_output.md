The complete creative direction, image generation prompts, and 300vh sticky scroll-reveal architecture have been written to `/Users/ajaytiwari/Desktop/Projects/courseo/testscript/neon_scroll_consult_output.md`.

### Summary of Deliverables Written

1. **Hero Cockpit Stage Prompts (16:9 Widescreen)**
   * **Stage 1 — The Prompt Plateau (`web/assets/hero/stage-1-prompt-plateau.webp`):** Brittle, isolated single-pane chat terminal in dark obsidian (`#0F172A`) with amber-orange alert glyphs (`#FD6902`), cold directional lighting, and stark negative space conveying 1x manual bottlenecks.
   * **Stage 2 — The Deep Skills Leap (`web/assets/hero/stage-2-deep-skills-leap.webp`):** Isometric 3D multi-agent swarm DAG suspended above a matte-black H100 80GB GPU cluster rack, with radiant tangerine (`#FD6902`) and telemetry cyan (`#06B6D4`) optical data pipes.
   * **Stage 3 — The 10x Superpower (`web/assets/hero/stage-3-10x-superpower.webp`):** Dark obsidian and brushed porcelain (`#F8FAFC`) executive deployment cockpit displaying verified CI/CD repositories, sub-50ms latency sparklines, and multi-region telemetry.

2. **Flagship Course Track Card Prompts (16:10 Landscape)**
   * **Autonomous Agent Swarms (`web/assets/tracks/track-agent-swarms.webp`):** Interconnected obsidian hexagonal agent nodes with radiant tangerine light beams and left-side negative space for card typography.
   * **High-Production Generative Cinema (`web/assets/tracks/track-generative-cinema.webp`):** Matte-black anamorphic lens intersecting a 4K diffusion latent-space prism with optical glass caustics.
   * **Local LLM Inference (`web/assets/tracks/track-local-llm.webp`):** Macro unified-memory silicon die and anodized heatsink fins with cyan and tangerine micro-traces for quantized MLX/vLLM execution.
   * **Algorithmic Growth & Meta Ads (`web/assets/tracks/track-algorithmic-growth.webp`):** Floating translucent creative frames converging into an ascending radiant tangerine ROAS curve and conversion telemetry.

3. **300vh Sticky Scroll-Pinned Reveal Architecture**
   * **Layout & Pinning (`web/courseo.css`):** A `300vh` `.scroll-story` outer track housing a `position: sticky; top: 0; height: 100vh;` viewport that locks during traversal and cleanly unpins at the bottom, with a `prefers-reduced-motion` fallback.
   * **Stage Transitions:** Stacked GPU-composited `.cockpit-stage` layers transitioning via `opacity` and `translate3d` / `scale` (`is-active` and `is-exited` states).
   * **Scroll Controller (`web/hero-story.js`):** `requestAnimationFrame`-throttled passive scroll listener that normalizes scroll progress (`0.0` to `1.0`) across `section.offsetHeight - window.innerHeight`, updates the progress bar, switches active stages/steps, and supports smooth-scroll click navigation on each stage rail item.

### Next Steps for Integration
* Generate the 7 `.webp` assets using the prompts in `testscript/neon_scroll_consult_output.md` and place them under `web/assets/hero/` and `web/assets/tracks/`.
* Wire the HTML markup into `web/index.html`, append the sticky scroll styles to `web/courseo.css`, and include `web/hero-story.js` before the closing `</body>` tag.

Status: success (code: 0) | Tokens: 640 | Cost: $0.0002 | Artifacts: 1
synchronized swarm topology over a dark slate grid (`#0F172A`). Thin radiant tangerine (`#FD6902`) and crisp white light beams connect tool-calling nodes in mid-execution, subtle glass refraction, clean negative space on the left for card typography, studio macro lighting, 8k Octane Render.

### Track 2: High-Production Generative Cinema
- **File Target:** `web/assets/tracks/track-generative-cinema.webp`
- **Prompt:**
  > Luxury cinema-grade 3D composition of an anamorphic matte-black camera lens intersecting with a glowing 4K diffusion latent-space prism. Volumetric light beams split into warm radiant tangerine (`#FD6902`) and cool tungsten highlights against a deep obsidian background (`#020617`). Subtle film-frame aspect markers, razor-sharp optical glass caustics, 8k Redshift render.

### Track 3: Local LLM Inference
- **File Target:** `web/assets/tracks/track-local-llm.webp`
- **Prompt:**
  > Macro industrial design render of a precision unified-memory silicon die and anodized dark-slate heatsink fins representing quantized 4-bit local LLM inference (MLX & vLLM). Crisp cyan (`#06B6D4`) and radiant tangerine (`#FD6902`) micro-traces glow subtly along the silicon pathways, zero cloud iconography, cool thermal studio lighting on obsidian backdrop, 8k.

### Track 4: Algorithmic Growth & Meta Ads
- **File Target:** `web/assets/tracks/track-algorithmic-growth.webp`
- **Prompt:**
  > Sleek 3D glassmorphic telemetry sculpture showing a live creative testing loop: three floating translucent ad-creative frames converging into an ascending radiant tangerine (`#FD6902`) ROAS performance curve and precision conversion counters. Dark obsidian (`#0F172A`) and porcelain (`#F8FAFC`) accents, high-contrast studio lighting, ultra-clean fintech/adtech aesthetic, 8k.

---

## 3. 300vh Sticky Scroll-Pinned Reveal Architecture

### HTML Structure (`web/index.html`)
```html
<section class="scroll-story" id="hero-scroll-story" aria-label="From Lagging Behind to 10x AI Architect">
  <div class="scroll-story__sticky">
    <div class="scroll-story__shell">
      <!-- Left/Top Narrative & Stage Rail -->
      <div class="scroll-story__copy">
        <div class="scroll-story__eyebrow">COURSEOS ARCHITECTURE</div>
        <ol class="scroll-story__steps" role="tablist">
          <li class="scroll-step is-active" data-stage="0">
            <span class="scroll-step__index">01</span>
            <div class="scroll-step__meta">
              <h3>The Prompt Plateau</h3>
              <p>Brittle single-turn chats and 1x manual bottlenecks.</p>
            </div>
          </li>
          <li class="scroll-step" data-stage="1">
            <span class="scroll-step__index">02</span>
            <div class="scroll-step__meta">
              <h3>The Deep Skills Leap</h3>
              <p>Multi-agent DAGs, H100 clusters, and deterministic evals.</p>
            </div>
          </li>
          <li class="scroll-step" data-stage="2">
            <span class="scroll-step__index">03</span>
            <div class="scroll-step__meta">
              <h3>The 10x Superpower</h3>
              <p>Production deployments, verified repos, live telemetry.</p>
            </div>
          </li>
        </ol>
        <div class="scroll-story__progress" aria-hidden="true">
          <div class="scroll-story__progress-fill" id="scroll-progress-fill"></div>
        </div>
      </div>

      <!-- Right/Center Pinned Visual Cockpit -->
      <div class="scroll-story__viewport">
        <figure class="cockpit-stage is-active" data-stage="0">
          <img src="assets/hero/stage-1-prompt-plateau.webp" alt="The Prompt Plateau isolated terminal" loading="eager" />
        </figure>
        <figure class="cockpit-stage" data-stage="1">
          <img src="assets/hero/stage-2-deep-skills-leap.webp" alt="Multi-agent swarm DAG and H100 GPU cluster" loading="lazy" />
        </figure>
        <figure class="cockpit-stage" data-stage="2">
          <img src="assets/hero/stage-3-10x-superpower.webp" alt="Production AI deployment dashboard" loading="lazy" />
        </figure>
      </div>
    </div>
  </div>
</section>
```

### CSS Architecture (`web/courseo.css`)
```css
:root {
  --courseo-accent: #FD6902;
  --courseo-obsidian: #0F172A;
  --courseo-porcelain: #F8FAFC;
  --courseo-cyan: #06B6D4;
}

/* 300vh outer scroll track enables 3 distinct pinned stages before unpinning */
.scroll-story {
  position: relative;
  height: 300vh;
  background: var(--courseo-obsidian);
  color: var(--courseo-porcelain);
}

/* Sticky viewport locks to 100vh during scroll traversal and cleanly unpins at bottom */
.scroll-story__sticky {
  position: sticky;
  top: 0;
  height: 100vh;
  width: 100%;
  overflow: hidden;
  display: flex;
  align-items: center;
}

.scroll-story__shell {
  width: min(1280px, 92vw);
  margin: 0 auto;
  display: grid;
  grid-template-columns: minmax(300px, 420px) 1fr;
  gap: clamp(2rem, 4vw, 4.5rem);
  align-items: center;
}

.scroll-story__viewport {
  position: relative;
  width: 100%;
  aspect-ratio: 16 / 9;
  border-radius: 20px;
  overflow: hidden;
  background: #020617;
  border: 1px solid rgba(248, 250, 252, 0.1);
  box-shadow: 0 32px 80px rgba(0, 0, 0, 0.55);
}

/* Stacked GPU-composited stage layers */
.cockpit-stage {
  position: absolute;
  inset: 0;
  margin: 0;
  opacity: 0;
  transform: translate3d(0, 28px, 0) scale(0.96);
  transition: opacity 480ms cubic-bezier(0.22, 1, 0.36, 1),
              transform 640ms cubic-bezier(0.22, 1, 0.36, 1);
  will-change: opacity, transform;
  pointer-events: none;
}

.cockpit-stage.is-active {
  opacity: 1;
  transform: translate3d(0, 0, 0) scale(1);
  pointer-events: auto;
}

.cockpit-stage.is-exited {
  opacity: 0;
  transform: translate3d(0, -24px, 0) scale(1.03);
}

.cockpit-stage img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.scroll-story__steps {
  list-style: none;
  padding: 0;
  margin: 1.5rem 0 2rem;
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.scroll-step {
  display: flex;
  gap: 1rem;
  padding: 1rem 1.25rem;
  border-radius: 12px;
  border-left: 2px solid rgba(248, 250, 252, 0.14);
  opacity: 0.45;
  transition: opacity 300ms ease, border-color 300ms ease, background-color 300ms ease;
  cursor: pointer;
}

.scroll-step.is-active {
  opacity: 1;
  border-left-color: var(--courseo-accent);
  background: rgba(253, 105, 2, 0.08);
}

.scroll-story__progress {
  height: 3px;
  width: 100%;
  background: rgba(248, 250, 252, 0.12);
  border-radius: 999px;
  overflow: hidden;
}

.scroll-story__progress-fill {
  height: 100%;
  width: 0%;
  background: linear-gradient(90deg, var(--courseo-accent), var(--courseo-cyan));
  transform-origin: left center;
  will-change: width;
}

@media (max-width: 960px) {
  .scroll-story__shell {
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }
}

@media (prefers-reduced-motion: reduce) {
  .scroll-story {
    height: auto;
    padding: 4rem 0;
  }
  .scroll-story__sticky {
    position: relative;
    height: auto;
  }
  .cockpit-stage {
    transition: none;
  }
}
```

### JavaScript Scroll-Pinning Controller (`web/hero-story.js`)
```javascript
(() => {
  const section = document.getElementById('hero-scroll-story');
  if (!section) return;

  const stages = Array.from(section.querySelectorAll('.cockpit-stage'));
  const steps = Array.from(section.querySelectorAll('.scroll-step'));
  const progressFill = document.getElementById('scroll-progress-fill');
  const totalStages = stages.length;
  let activeIndex = 0;
  let rafId = null;

  const clamp = (val, min, max) => Math.min(Math.max(val, min), max);

  function setStage(index) {
    if (index === activeIndex) return;
    activeIndex = index;

    stages.forEach((stage, i) => {
      stage.classList.toggle('is-active', i === index);
      stage.classList.toggle('is-exited', i < index);
    });

    steps.forEach((step, i) => {
      step.classList.toggle('is-active', i === index);
      step.setAttribute('aria-selected', String(i === index));
    });
  }

  function updateOnScroll() {
    rafId = null;
    const rect = section.getBoundingClientRect();
    const scrollableDistance = section.offsetHeight - window.innerHeight;
    if (scrollableDistance <= 0) return;

    // Normalized progress [0.0 -> 1.0] across the 300vh track
    const rawProgress = -rect.top / scrollableDistance;
    const progress = clamp(rawProgress, 0, 1);

    if (progressFill) {
      progressFill.style.width = `${(progress * 100).toFixed(2)}%`;
    }

    // Map [0, 1] into discrete stage indices [0, 1, 2]
    const targetIndex = Math.min(
      totalStages - 1,
      Math.floor(progress * totalStages)
    );
    setStage(targetIndex);
  }

  function onScroll() {
    if (rafId === null) {
      rafId = window.requestAnimationFrame(updateOnScroll);
    }
  }

  // Allow clicking any step to smoothly scroll to that stage's pinned offset
  steps.forEach((step, i) => {
    step.addEventListener('click', () => {
      const scrollableDistance = section.offsetHeight - window.innerHeight;
      const targetTop = section.offsetTop + (scrollableDistance * (i / totalStages)) + 8;
      window.scrollTo({ top: targetTop, behavior: 'smooth' });
    });
  });

  window.addEventListener('scroll', onScroll, { passive: true });
  window.addEventListener('resize', onScroll, { passive: true });
  updateOnScroll();
})();
```
