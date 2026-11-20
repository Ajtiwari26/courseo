
## 1. Executive Architectural Recommendation

### Scroll Concept Evaluation

| Concept | Visual Signature | 60fps Feasibility | Engineering Trade-off |
| :--- | :--- | :--- | :--- |
| **1. 3D Perspective Untwist & Unfold** (Linear / Stripe) | Hero console starts pitched in 3D space (`rotateX`/`rotateY`), flattens into a full-bleed command center as you scroll, while flanking metric cards fan out. | **Guaranteed 60fps** (pure GPU compositor `transform` & `opacity`). | Requires disciplined `will-change` scoping and a `220vh–240vh` sticky runway. |
| **2. Scroll-Scrubbed Motion Reveal Canvas** (Apple) | Scrubs `courseo_motion_reveal.mp4` or `motion_frame_01..05.jpg` directly against scroll progress. | **High risk** with raw MP4 seeking (Safari/mobile decode stalls); 5 JPG frames are too sparse for smooth 60fps scrubbing. | Best used as a **hybrid**: autoplay `courseo_motion_reveal.mp4` inside the 3D stage with `motion_frame_05.jpg` as the crisp poster/fallback. |
| **3. Dual-Axis Parallax Floating Console + Live Gauge Spin-Up** | Foreground console and background telemetry cards move at distinct Z-depths; SVG dial gauges animate on lock-in. | **Guaranteed 60fps** when restricted to `translate3d` and `stroke-dashoffset`. | Works best layered on top of Concept 1 rather than standalone. |

**Recommended Architecture:** Adopt **Concept 1 (3D Perspective Untwist & Unfold)** as the primary scroll engine, embed **`courseo_motion_reveal.mp4`** inside the hero's central showcase frame (eliminating video-scrub decode jank), and layer **Concept 3's** cursor-tracked specular sheen and SVG telemetry gauge spin-up at scroll lock-in. All scroll physics drive CSS custom properties (`--p`, `--mx`, `--my`) from a single `requestAnimationFrame` loop gated by `IntersectionObserver`.

---

## 2. Asset Elevation Blueprint

Replace every placeholder initial badge and generic card background with structured, lazy-loaded media backed by deterministic aspect ratios (zero Cumulative Layout Shift).

| Surface | Current Placeholder | Target Asset & Path | Rendering & Polish Strategy |
| :--- | :--- | :--- | :--- |
| **Navbar & Hero Brand Mark** | Static icon / text | `web/assets/courseo_logo_official.jpg` | Circular/squircle clip (`border-radius: 10px`), `object-fit: cover`, subtle `1px solid rgba(255,255,255,0.12)` inner ring. |
| **Hero Central Reveal** | Static split block | `web/assets/courseo_motion_reveal.mp4` + `poster="assets/brand/motion_frame_05.jpg"` | `autoplay muted loop playsinline preload="metadata"`; pauses automatically via `IntersectionObserver` when scrolled off-screen to save GPU/battery. |
| **Brand Storyboard Strip** | None | `web/assets/brand/motion_frame_01.jpg` – `motion_frame_05.jpg` | 5-frame horizontal filmstrip below hero or inside the "How Courseo Works" bento card with hover frame-focus scaling. |
| **Faculty Portraits** | `"RV"`, `"AN"`, `"KM"`, `"DS"` | `web/assets/faculty/{rv,an,km,ds}.webp` | `aspect-ratio: 4 / 5`, `object-fit: cover; object-position: top center`. Subtle `grayscale(25%) contrast(1.04)` at rest transitioning to full color on card hover. |
| **Student Avatar Stacks** | `"AK"`, `"SR"` | `web/assets/students/{ak,sr,03,04}.webp` | `32px × 32px` overlapping stack (`margin-left: -10px`), `2px solid var(--bg-surface)` cutout ring, paired with a tabular-nums cohort pill (`+1,420 Builders`). |
| **Capstone: OpenClaw DAG Runner** | Generic card | `web/assets/capstones/openclaw-dag.webp` | 16:10 dark-mode node-graph preview with overlaid live status pill (`14 nodes · 42ms p99`) and specular hover border. |
| **Capstone: Seedance 2.5 Cinema** | Generic card | `web/assets/capstones/seedance-timeline.webp` | 16:10 multi-track generative timeline preview with subtle animated playhead scrubber (`translateX`). |
| **Capstone: Meta Ads ₹1.2 Cr Scale** | Generic card | `web/assets/capstones/meta-ads-telemetry.webp` | 16:10 ROAS & spend-curve dashboard preview overlaid with live animated `₹1.2 Cr` / `4.8x ROAS` telemetry badge. |

---

## 3. The Hero Scroll Interaction Specification

The hero uses a **sticky scroll runway** (`height: 240vh`) containing a `position: sticky; top: 0; height: 100vh` viewport stage. Scroll progress `p` is normalized to `[0.0, 1.0]`.

| Phase | Scroll Range (`p`) | Visual Transformation | GPU Properties Animated |
| :--- | :--- | :--- | :--- |
| **Phase 0: Isometric Entry** | `0.00 → 0.15` | Headline and CTA sit crisply above the fold. The central Courseo console rests pitched back at `rotateX(18deg) rotateY(-6deg) scale(0.90)` with `courseo_motion_reveal.mp4` playing inside. | `transform: perspective(1200px) rotateX(...) rotateY(...) scale(...)` |
| **Phase 1: Untwist & Unfold** | `0.15 → 0.55` | As the user scrolls, the console smoothly untwists to `rotateX(0deg) rotateY(0deg) scale(1.0)`. Left and right floating telemetry cards (`OpenClaw DAG`, `₹1.2 Cr ROAS`) fan outward from behind the console along `translate3d(±X, Y, 0)`. | `transform: translate3d(...)`, `opacity` |
| **Phase 2: Lock-In & Gauge Spin-Up** | `0.55 → 0.85` | Console locks flat in the center viewport. The SVG radial dial gauge triggers its `stroke-dashoffset` ease-in (`cubic-bezier(0.16, 1, 0.3, 1)`), and numeric readouts settle. | `stroke-dashoffset`, `--gauge-progress` |
| **Phase 3: Clean Handoff** | `0.85 → 1.00` | Stage glides upward (`translate3d(0, -40px, 0)`) and hands off cleanly to the Faculty & Capstones section with zero layout shift. | `transform: translate3d(...)`, `opacity` |

**Micro-Interactions:**
* **Specular Reflection:** A `radial-gradient` pseudo-element (`::before`) tracks pointer coordinates (`--mx`, `--my`) across the console bezel and capstone cards.
* **Subtle Cursor Tilt:** When `p > 0.50` (console flattened), pointer offset applies a damped `±2.5deg` `rotateX`/`rotateY` tilt.
* **Reduced Motion:** `@media (prefers-reduced-motion: reduce)` collapses the runway to `height: auto`, disables 3D pitch, and renders the flat console with static `motion_frame_05.jpg`.

---

## 4. Concrete Implementation Plan for `web/index.html` & `web/courseo.css`

### A. `web/index.html` — Hero Scroll Stage & Asset Markup

Insert the sticky scroll runway in the hero section and upgrade the avatar/faculty/capstone blocks:

```html
<!-- HERO SCROLL RUNWAY -->
<section class="hero-runway" id="hero-runway" aria-label="Courseo Interactive Showcase">
  <div class="hero-sticky">
    <header class="hero-copy">
      <div class="hero-badge">
        <img src="assets/courseo_logo_official.jpg" alt="Courseo" class="brand-mark" width="24" height="24" />
        <span>Cohort 04 · Live Systems &amp; AI Engineering</span>
      </div>
      <h1>Build Production Systems.<br />Not Toy Demos.</h1>
      <p class="hero-sub">
        Master autonomous DAGs, generative cinema pipelines, and ₹1.2 Cr+ growth telemetry with practitioners.
      </p>
      <div class="hero-actions">
        <a href="#apply" class="btn-primary">Apply for Cohort</a>
        <div class="student-stack" aria-label="Active cohort builders">
          <img src="assets/students/ak.webp" alt="Aarav K." width="32" height="32" loading="eager" />
          <img src="assets/students/sr.webp" alt="Sneha R." width="32" height="32" loading="eager" />
          <img src="assets/students/03.webp" alt="Rohan M." width="32" height="32" loading="eager" />
          <img src="assets/students/04.webp" alt="Meera V." width="32" height="32" loading="eager" />
          <span class="stack-count">+1,420 builders</span>
        </div>
      </div>
    </header>

    <!-- 3D STAGE -->
    <div class="hero-stage" id="hero-stage">
      <!-- Left Floating Telemetry Card -->
      <aside class="float-card float-left" aria-label="OpenClaw DAG Telemetry">
        <span class="card-kicker">OpenClaw DAG Runner</span>
        <strong class="metric-val">42ms <small>p99</small></strong>
        <img src="assets/capstones/openclaw-dag.webp" alt="OpenClaw DAG execution graph" width="240" height="150" loading="lazy" />
      </aside>

      <!-- Central Unfolding Console -->
      <div class="hero-console specular-surface" id="hero-console">
        <div class="console-chrome">
          <span class="dot"></span><span class="dot"></span><span class="dot"></span>
          <span class="console-title">courseo.runtime // live-reveal</span>
        </div>
        <div class="console-viewport">
          <video
            id="hero-reveal-video"
            src="assets/courseo_motion_reveal.mp4"
            poster="assets/brand/motion_frame_05.jpg"
            autoplay
            muted
            loop
            playsinline
            preload="metadata"
          ></video>
        </div>
      </div>

      <!-- Right Floating Telemetry Card with SVG Gauge -->
      <aside class="float-card float-right" aria-label="Meta Ads Scale Telemetry">
        <span class="card-kicker">Meta Ads Scale</span>
        <div class="gauge-row">
          <svg class="dial-gauge" viewBox="0 0 64 64" width="56" height="56" aria-hidden="true">
            <circle class="gauge-track" cx="32" cy="32" r="26" />
            <circle class="gauge-fill" cx="32" cy="32" r="26" />
          </svg>
          <div>
            <strong class="metric-val">₹1.2 Cr</strong>
            <span class="metric-sub">4.8x Verified ROAS</span>
          </div>
        </div>
      </aside>
    </div>
  </div>
</section>
```

### B. `web/courseo.css` — GPU-Composited 3D Transforms & Specular Polish

```css
:root {
  --p: 0;           /* Normalized scroll progress [0..1] */
  --unfold: 0;      /* Phase 1 progress [0..1] */
  --gauge: 0;       /* Phase 2 gauge trigger [0..1] */
  --mx: 50%;        /* Pointer X for specular sheen */
  --my: 50%;        /* Pointer Y for specular sheen */
}

.hero-runway {
  position: relative;
  height: 230vh;
}

.hero-sticky {
  position: sticky;
  top: 0;
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  overflow: hidden;
  padding: 2rem 1.5rem;
}

/* 3D Perspective Stage */
.hero-stage {
  position: relative;
  width: min(980px, 92vw);
  margin-top: 2rem;
  perspective: 1200px;
  transform-style: preserve-3d;
}

.hero-console {
  position: relative;
  border-radius: 16px;
  background: #0d0f14;
  border: 1px solid rgba(255, 255, 255, 0.12);
  overflow: hidden;
  box-shadow: 0 32px 80px rgba(0, 0, 0, 0.55);
  will-change: transform;
  transform:
    rotateX(calc((1 - var(--unfold)) * 18deg))
    rotateY(calc((1 - var(--unfold)) * -6deg))
    scale(calc(0.90 + var(--unfold) * 0.10));
}

.console-viewport {
  aspect-ratio: 16 / 9;
  background: #07080b;
}

.console-viewport video {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

/* Floating Telemetry Cards */
.float-card {
  position: absolute;
  top: 50%;
  width: 240px;
  padding: 14px;
  border-radius: 12px;
  background: rgba(15, 18, 25, 0.86);
  backdrop-filter: blur(14px);
  border: 1px solid rgba(255, 255, 255, 0.12);
  will-change: transform, opacity;
  opacity: calc(0.15 + var(--unfold) * 0.85);
}

.float-left {
  left: -48px;
  transform: translate3d(calc((1 - var(--unfold)) * 72px), calc(-50% + (1 - var(--unfold)) * 24px), 40px);
}

.float-right {
  right: -48px;
  transform: translate3d(calc((1 - var(--unfold)) * -72px), calc(-50% - (1 - var(--unfold)) * 20px), 40px);
}

/* Specular Hover Reflection */
.specular-surface::before {
  content: "";
  position: absolute;
  inset: 0;
  pointer-events: none;
  background: radial-gradient(
    420px circle at var(--mx) var(--my),
    rgba(255, 255, 255, 0.12),
    transparent 60%
  );
  z-index: 2;
}

/* SVG Dial Gauge Spin-Up (circumference = 2 * pi * 26 ≈ 163.36) */
.dial-gauge {
  transform: rotate(-90deg);
}

.gauge-track {
  fill: none;
  stroke: rgba(255, 255, 255, 0.1);
  stroke-width: 5;
}

.gauge-fill {
  fill: none;
  stroke: #f05032;
  stroke-width: 5;
  stroke-linecap: round;
  stroke-dasharray: 163.36;
  stroke-dashoffset: calc(163.36 * (1 - var(--gauge) * 0.84));
  transition: stroke-dashoffset 0.6s cubic-bezier(0.16, 1, 0.3, 1);
}

/* Student Avatar Stack & Faculty Portraits */
.student-stack {
  display: inline-flex;
  align-items: center;
}

.student-stack img {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid #0b0d12;
  margin-left: -10px;
}

.student-stack img:first-child {
  margin-left: 0;
}

.faculty-portrait {
  width: 100%;
  aspect-ratio: 4 / 5;
  object-fit: cover;
  object-position: top center;
  border-radius: 12px;
  filter: grayscale(25%) contrast(1.04);
  transition: filter 0.35s ease, transform 0.35s cubic-bezier(0.16, 1, 0.3, 1);
}

.faculty-card:hover .faculty-portrait {
  filter: grayscale(0%) contrast(1.02);
  transform: scale(1.02);
}

/* Accessibility & Mobile Fallback */
@media (max-width: 900px) {
  .float-card { display: none; }
}

@media (prefers-reduced-motion: reduce) {
  .hero-runway { height: auto; }
  .hero-sticky { position: static; height: auto; }
  .hero-console, .float-card { transform: none !important; opacity: 1 !important; }
  .gauge-fill { stroke-dashoffset: 26; }
}
```

### C. Zero-Jank Scroll & Specular Controller (`web/hero_scroll.js`)

Attach this lightweight script at the end of `web/index.html`. It writes only CSS custom properties inside `requestAnimationFrame` and pauses both the scroll listener and `<video>` when the hero leaves the viewport.

```javascript
(() => {
  const runway = document.getElementById("hero-runway");
  const consoleEl = document.getElementById("hero-console");
  const videoEl = document.getElementById("hero-reveal-video");
  if (!runway || window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;

  let active = true;
  let rafId = 0;

  const clamp01 = (v) => (v < 0 ? 0 : v > 1 ? 1 : v);

  const updateScroll = () => {
    rafId = 0;
    const rect = runway.getBoundingClientRect();
    const scrollable = runway.offsetHeight - window.innerHeight;
    if (scrollable <= 0) return;

    const p = clamp01(-rect.top / scrollable);
    const unfold = clamp01((p - 0.12) / 0.43);
    const gauge = p >= 0.55 ? 1 : 0;

    runway.style.setProperty("--p", p.toFixed(4));
    runway.style.setProperty("--unfold", unfold.toFixed(4));
    runway.style.setProperty("--gauge", String(gauge));
  };

  const onScroll = () => {
    if (active && !rafId) rafId = requestAnimationFrame(updateScroll);
  };

  const io = new IntersectionObserver(
    ([entry]) => {
      active = entry.isIntersecting;
      if (videoEl) {
        if (active) videoEl.play().catch(() => {});
        else videoEl.pause();
      }
      if (active) onScroll();
    },
    { threshold: 0 }
  );
  io.observe(runway);

  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", onScroll, { passive: true });

  if (consoleEl) {
    consoleEl.addEventListener("pointermove", (e) => {
      const r = consoleEl.getBoundingClientRect();
      consoleEl.style.setProperty("--mx", `${((e.clientX - r.left) / r.width) * 100}%`);
      consoleEl.style.setProperty("--my", `${((e.clientY - r.top) / r.height) * 100}%`);
    });
  }

  updateScroll();
})();
```

Status: success (code: 0) | Tokens: 760 | Cost: $0.0001 | Artifacts: 1
