
## A. HTML Component Structure (`web/index.html`)

Place this component inside the `<main>` container of `web/index.html` as the central Hero Stage.

```html
<section
  id="hero-transformation"
  class="courseo-hero"
  aria-labelledby="hero-headline"
  data-stage="0"
>
  <div class="courseo-hero__container">
    <!-- Left / Top Narrative Header -->
    <header class="courseo-hero__header">
      <div class="courseo-hero__eyebrow">
        <span class="courseo-hero__pulse" aria-hidden="true"></span>
        <span>From Lagging Behind to 10x AI Architect</span>
      </div>

      <h1 id="hero-headline" class="courseo-hero__title">
        Stop typing surface-level prompts.
        <span class="courseo-hero__title-accent">Start building deep AI systems.</span>
      </h1>

      <p class="courseo-hero__subtitle">
        Feeling like everyone at work is shipping faster while you are stuck in chat threads?
        Prompts are a commodity. Experience the shift from manual busywork to production AI engineering.
      </p>

      <div class="courseo-hero__actions">
        <a href="#tracks" class="courseo-btn courseo-btn--primary">
          Start Building Free
          <span aria-hidden="true">→</span>
        </a>
        <button
          type="button"
          class="courseo-btn courseo-btn--ghost"
          id="hero-autoplay-toggle"
          aria-pressed="true"
        >
          <span class="courseo-btn__dot" aria-hidden="true"></span>
          <span data-autoplay-label>Pause Story</span>
        </button>
      </div>
    </header>

    <!-- Central Interactive Storytelling Stage -->
    <div class="hero-stage" id="hero-story-stage">
      <!-- 3-Step Interactive Tabs with Live Progress Bars -->
      <div class="hero-stage__tabs" role="tablist" aria-label="AI Transformation Stages">
        <button
          type="button"
          role="tab"
          id="tab-stage-0"
          class="stage-tab is-active"
          aria-selected="true"
          aria-controls="panel-stage-0"
          data-stage-index="0"
        >
          <span class="stage-tab__meta">
            <span class="stage-tab__step">01</span>
            <span class="stage-tab__badge">1x Leverage</span>
          </span>
          <span class="stage-tab__title">The Plateau</span>
          <span class="stage-tab__desc">Lagging behind in chat prompts</span>
          <span class="stage-tab__progress-track" aria-hidden="true">
            <span class="stage-tab__progress-fill" data-progress-for="0"></span>
          </span>
        </button>

        <button
          type="button"
          role="tab"
          id="tab-stage-1"
          class="stage-tab"
          aria-selected="false"
          aria-controls="panel-stage-1"
          data-stage-index="1"
        >
          <span class="stage-tab__meta">
            <span class="stage-tab__step">02</span>
            <span class="stage-tab__badge">Deep Skills</span>
          </span>
          <span class="stage-tab__title">The Skills Unlock</span>
          <span class="stage-tab__desc">Swarms, 4K cinema &amp; fine-tuning</span>
          <span class="stage-tab__progress-track" aria-hidden="true">
            <span class="stage-tab__progress-fill" data-progress-for="1"></span>
          </span>
        </button>

        <button
          type="button"
          role="tab"
          id="tab-stage-2"
          class="stage-tab"
          aria-selected="false"
          aria-controls="panel-stage-2"
          data-stage-index="2"
        >
          <span class="stage-tab__meta">
            <span class="stage-tab__step">03</span>
            <span class="stage-tab__badge stage-tab__badge--accent">10x Leverage</span>
          </span>
          <span class="stage-tab__title">The Superpower</span>
          <span class="stage-tab__desc">Courseo production reality</span>
          <span class="stage-tab__progress-track" aria-hidden="true">
            <span class="stage-tab__progress-fill" data-progress-for="2"></span>
          </span>
        </button>
      </div>

      <!-- Stage Viewport: Narrative + Code & Telemetry Mockup -->
      <div class="hero-stage__viewport">
        <!-- STAGE 1: THE PLATEAU / LAGGING BEHIND -->
        <article
          id="panel-stage-0"
          class="stage-panel is-active"
          role="tabpanel"
          aria-labelledby="tab-stage-0"
          data-panel-index="0"
        >
          <div class="stage-panel__story">
            <span class="stage-pill stage-pill--muted">Stage 01 • The Plateau</span>
            <h2 class="stage-panel__heading">
              4.5 hours lost daily to copy-pasting surface-level prompts.
            </h2>
            <p class="stage-panel__copy">
              You open a chat window, type a one-line prompt, edit generic output, and repeat.
              Meanwhile, peers who understand underlying AI architecture are automating entire
              workflows before lunch.
            </p>

            <dl class="telemetry-grid">
              <div class="telemetry-card">
                <dt>Output Leverage</dt>
                <dd class="telemetry-card__value">1.0x</dd>
                <span class="telemetry-card__sub">Linear manual effort</span>
              </div>
              <div class="telemetry-card telemetry-card--warn">
                <dt>Daily Busywork Lost</dt>
                <dd class="telemetry-card__value">4.5 hrs</dd>
                <span class="telemetry-card__sub">Copy-paste &amp; rework</span>
              </div>
              <div class="telemetry-card">
                <dt>Workflow Depth</dt>
                <dd class="telemetry-card__value">Ad-hoc</dd>
                <span class="telemetry-card__sub">Single-turn chat prompts</span>
              </div>
            </dl>
          </div>

          <div class="stage-panel__artifact" aria-label="Surface-level chat prompt mockup">
            <div class="artifact-window artifact-window--muted">
              <div class="artifact-window__bar">
                <span class="window-dots" aria-hidden="true"><i></i><i></i><i></i></span>
                <span class="artifact-window__title">generic-chat-thread.txt — 14th tab open</span>
                <span class="artifact-status artifact-status--idle">MANUAL LOOP</span>
              </div>
              <div class="artifact-window__body">
                <div class="chat-bubble chat-bubble--user">
                  <span class="chat-bubble__role">You (09:42 AM)</span>
                  <p>"Write a product launch campaign and summarize these 40 support tickets."</p>
                </div>
                <div class="chat-bubble chat-bubble--bot">
                  <span class="chat-bubble__role">Generic Chat</span>
                  <p>
                    "Sure! Here is a high-level draft. Note: I cannot access your live repo,
                    brand assets, or ad metrics—please paste each section manually..."
                  </p>
                </div>
                <div class="artifact-footer-log">
                  <span>⚠ Context window truncated</span>
                  <span>⏱ 4.5 hrs/day manual glue work</span>
                </div>
              </div>
            </div>
          </div>
        </article>

        <!-- STAGE 2: THE SKILLS UNLOCK -->
        <article
          id="panel-stage-1"
          class="stage-panel"
          role="tabpanel"
          aria-labelledby="tab-stage-1"
          data-panel-index="1"
          hidden
        >
          <div class="stage-panel__story">
            <span class="stage-pill stage-pill--dark">Stage 02 • The Skills Unlock</span>
            <h2 class="stage-panel__heading">
              Prompts are a commodity. Deep systems are your moat.
            </h2>
            <p class="stage-panel__copy">
              You leave the chatbox behind and master the production stack: orchestrating
              multi-agent swarms, directing 4K generative cinema pipelines, wiring automated
              ad systems, and fine-tuning local models on proprietary data.
            </p>

            <ul class="skill-chips" aria-label="Core deep AI skills">
              <li class="skill-chip">Multi-Agent Swarms</li>
              <li class="skill-chip">4K Generative Cinema</li>
              <li class="skill-chip">Automated Ad Systems</li>
              <li class="skill-chip">Local Model Fine-Tuning</li>
            </ul>

            <dl class="telemetry-grid">
              <div class="telemetry-card">
                <dt>Pipeline Execution</dt>
                <dd class="telemetry-card__value">Parallel</dd>
                <span class="telemetry-card__sub">4 autonomous agents</span>
              </div>
              <div class="telemetry-card">
                <dt>Model Adaptation</dt>
                <dd class="telemetry-card__value">QLoRA</dd>
                <span class="telemetry-card__sub">Local weights fine-tuned</span>
              </div>
              <div class="telemetry-card">
                <dt>Creative Render</dt>
                <dd class="telemetry-card__value">4K 60fps</dd>
                <span class="telemetry-card__sub">Deterministic seed graph</span>
              </div>
            </dl>
          </div>

          <div class="stage-panel__artifact" aria-label="Multi-agent and fine-tuning code mockup">
            <div class="artifact-window">
              <div class="artifact-window__bar">
                <span class="window-dots" aria-hidden="true"><i></i><i></i><i></i></span>
                <span class="artifact-window__title">swarm_pipeline.py — Courseo Lab</span>
                <span class="artifact-status artifact-status--active">COMPILING DAG</span>
              </div>
              <pre class="code-block"><code><span class="tok-kw">from</span> courseo.agents <span class="tok-kw">import</span> Swarm, ToolNode
<span class="tok-kw">from</span> courseo.models <span class="tok-kw">import</span> LocalLoRA, Cinema4K

<span class="tok-cm"># Fine-tuned local weights + deterministic 4K asset graph</span>
model = LocalLoRA.load(<span class="tok-str">"./weights/brand-v4-qlora"</span>, quant=<span class="tok-str">"4bit"</span>)

pipeline = Swarm(
    nodes=[
        ToolNode(<span class="tok-str">"research_agent"</span>, model=model),
        ToolNode(<span class="tok-str">"cinema_director"</span>, engine=Cinema4K(res=<span class="tok-str">"3840x2160"</span>)),
        ToolNode(<span class="tok-str">"ad_optimizer"</span>, auto_ab_test=<span class="tok-kw">True</span>),
    ],
    eval_gate=<span class="tok-num">0.94</span>
)

pipeline.deploy(sandbox=<span class="tok-str">"gpu-a100-cluster"</span>)</code></pre>
            </div>
          </div>
        </article>

        <!-- STAGE 3: THE SUPERPOWER / COURSEO REALITY -->
        <article
          id="panel-stage-2"
          class="stage-panel"
          role="tabpanel"
          aria-labelledby="tab-stage-2"
          data-panel-index="2"
          hidden
        >
          <div class="stage-panel__story">
            <span class="stage-pill stage-pill--accent">Stage 03 • The Superpower</span>
            <h2 class="stage-panel__heading">
              10x AI Architect. Shipped repos, cloud GPUs, undeniable proof.
            </h2>
            <p class="stage-panel__copy">
              No passive video playlists. You graduate with 4 production capstone repositories
              running on dedicated cloud GPU sandboxes—backed by the cohort rigor that drives
              a 94.6% completion rate and a +142% career boost.
            </p>

            <dl class="telemetry-grid telemetry-grid--highlight">
              <div class="telemetry-card telemetry-card--accent">
                <dt>Leverage Multiplier</dt>
                <dd class="telemetry-card__value">10x</dd>
                <span class="telemetry-card__sub">Autonomous output</span>
              </div>
              <div class="telemetry-card">
                <dt>Career Boost</dt>
                <dd class="telemetry-card__value">+142%</dd>
                <span class="telemetry-card__sub">Verified alumni lift</span>
              </div>
              <div class="telemetry-card">
                <dt>Cohort Completion</dt>
                <dd class="telemetry-card__value">94.6%</dd>
                <span class="telemetry-card__sub">4 capstone repos shipped</span>
              </div>
            </dl>
          </div>

          <div class="stage-panel__artifact" aria-label="Production telemetry and capstone repository dashboard">
            <div class="artifact-window artifact-window--accent">
              <div class="artifact-window__bar">
                <span class="window-dots" aria-hidden="true"><i></i><i></i><i></i></span>
                <span class="artifact-window__title">courseo-cloud://gpu-sandbox-prod</span>
                <span class="artifact-status artifact-status--live">● LIVE 10x</span>
              </div>
              <div class="artifact-window__body">
                <div class="repo-list">
                  <div class="repo-row">
                    <div>
                      <strong>01 / multi-agent-ops-swarm</strong>
                      <span>FastAPI • LangGraph • Eval 97.2%</span>
                    </div>
                    <span class="repo-tag">DEPLOYED</span>
                  </div>
                  <div class="repo-row">
                    <div>
                      <strong>02 / gen-cinema-4k-pipeline</strong>
                      <span>ComfyUI Graph • ControlNet • 4K</span>
                    </div>
                    <span class="repo-tag">DEPLOYED</span>
                  </div>
                  <div class="repo-row">
                    <div>
                      <strong>03 / automated-ad-growth-engine</strong>
                      <span>Multi-variant ROAS Loop • Live API</span>
                    </div>
                    <span class="repo-tag">DEPLOYED</span>
                  </div>
                  <div class="repo-row">
                    <div>
                      <strong>04 / local-llm-domain-finetune</strong>
                      <span>QLoRA • vLLM Inference • 18ms TTFT</span>
                    </div>
                    <span class="repo-tag">DEPLOYED</span>
                  </div>
                </div>
                <div class="sandbox-bar">
                  <span>Cloud GPU Sandbox: <strong>H100 80GB Ready</strong></span>
                  <span class="sandbox-bar__metric">4/4 Capstones Verified</span>
                </div>
              </div>
            </div>
          </div>
        </article>
      </div>

      <!-- Interactive Timeline Scrubber -->
      <div class="hero-scrubber">
        <div class="hero-scrubber__labels" aria-hidden="true">
          <span>1x Lagging Behind</span>
          <span>Deep Skills Unlock</span>
          <span>10x AI Architect</span>
        </div>
        <div class="hero-scrubber__control">
          <input
            type="range"
            id="hero-story-scrubber"
            class="hero-scrubber__input"
            min="0"
            max="100"
            step="1"
            value="0"
            aria-label="Scrub through the transformation from lagging behind to 10x AI Architect"
          />
          <div class="hero-scrubber__ticks" aria-hidden="true">
            <span data-tick="0"></span>
            <span data-tick="1"></span>
            <span data-tick="2"></span>
          </div>
        </div>
      </div>
    </div>
  </div>
</section>
```

---

## B. CSS Styling (`web/courseo.css`)

Uses Courseo's exact brand tokens (`#FD6902`, `#0F172A`, `#FFFFFF`, `#F8FAFC`) with crisp contrast, responsive layout, and reduced-motion support.

```css
/* ==========================================================================
   Courseo Brand Tokens & Hero Central Stage
   ========================================================================== */
:root {
  --courseo-tangerine: #FD6902;
  --courseo-tangerine-hover: #E05A00;
  --courseo-tangerine-soft: rgba(253, 105, 2, 0.1);
  --courseo-tangerine-border: rgba(253, 105, 2, 0.32);

  --courseo-obsidian: #0F172A;
  --courseo-obsidian-elevated: #1E293B;
  --courseo-slate-muted: #475569;
  --courseo-slate-subtle: #94A3B8;

  --courseo-porcelain: #FFFFFF;
  --courseo-snow: #F8FAFC;
  --courseo-border: #E2E8F0;

  --radius-sm: 8px;
  --radius-md: 14px;
  --radius-lg: 20px;
  --shadow-stage: 0 24px 60px -20px rgba(15, 23, 42, 0.14);
  --font-sans: "Inter", system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  --font-mono: "JetBrains Mono", "Fira Code", ui-monospace, SFMono-Regular, monospace;
}

.courseo-hero {
  background:
    radial-gradient(circle at 85% 10%, rgba(253, 105, 2, 0.08), transparent 42%),
    linear-gradient(180deg, var(--courseo-snow) 0%, var(--courseo-porcelain) 100%);
  color: var(--courseo-obsidian);
  font-family: var(--font-sans);
  padding: clamp(3rem, 6vw, 5.5rem) 1.25rem;
}

.courseo-hero__container {
  max-width: 1180px;
  margin: 0 auto;
}

/* Header */
.courseo-hero__header {
  max-width: 780px;
  margin: 0 auto 2.75rem;
  text-align: center;
}

.courseo-hero__eyebrow {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.4rem 0.9rem;
  border-radius: 999px;
  background: var(--courseo-porcelain);
  border: 1px solid var(--courseo-border);
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--courseo-obsidian);
  margin-bottom: 1.25rem;
}

.courseo-hero__pulse {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--courseo-tangerine);
  box-shadow: 0 0 0 4px var(--courseo-tangerine-soft);
}

.courseo-hero__title {
  font-size: clamp(2.1rem, 4.6vw, 3.6rem);
  line-height: 1.08;
  letter-spacing: -0.03em;
  font-weight: 800;
  margin: 0 0 1rem;
  color: var(--courseo-obsidian);
}

.courseo-hero__title-accent {
  display: block;
  color: var(--courseo-tangerine);
}

.courseo-hero__subtitle {
  font-size: clamp(1rem, 1.5vw, 1.15rem);
  line-height: 1.6;
  color: var(--courseo-slate-muted);
  margin: 0 auto 1.75rem;
  max-width: 640px;
}

.courseo-hero__actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 0.75rem;
}

.courseo-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.8rem 1.4rem;
  border-radius: 999px;
  font-size: 0.9375rem;
  font-weight: 600;
  text-decoration: none;
  cursor: pointer;
  border: 1px solid transparent;
  transition: transform 0.18s ease, background-color 0.18s ease, border-color 0.18s ease;
}

.courseo-btn--primary {
  background: var(--courseo-tangerine);
  color: var(--courseo-porcelain);
}

.courseo-btn--primary:hover {
  background: var(--courseo-tangerine-hover);
  transform: translateY(-1px);
}

.courseo-btn--ghost {
  background: var(--courseo-porcelain);
  color: var(--courseo-obsidian);
  border-color: var(--courseo-border);
}

.courseo-btn--ghost:hover {
  border-color: var(--courseo-obsidian);
}

.courseo-btn__dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--courseo-tangerine);
}

.courseo-btn--ghost[aria-pressed="false"] .courseo-btn__dot {
  background: var(--courseo-slate-subtle);
}

/* Central Interactive Stage */
.hero-stage {
  background: var(--courseo-porcelain);
  border: 1px solid var(--courseo-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-stage);
  overflow: hidden;
}

/* 3-Step Tabs */
.hero-stage__tabs {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  background: var(--courseo-snow);
  border-bottom: 1px solid var(--courseo-border);
}

.stage-tab {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.25rem;
  padding: 1.1rem 1.35rem 1.25rem;
  background: transparent;
  border: 0;
  border-right: 1px solid var(--courseo-border);
  text-align: left;
  cursor: pointer;
  color: var(--courseo-slate-muted);
  transition: background-color 0.2s ease, color 0.2s ease;
}

.stage-tab:last-child {
  border-right: 0;
}

.stage-tab:hover {
  background: rgba(255, 255, 255, 0.7);
}

.stage-tab.is-active {
  background: var(--courseo-porcelain);
  color: var(--courseo-obsidian);
}

.stage-tab__meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  margin-bottom: 0.2rem;
}

.stage-tab__step {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  font-weight: 700;
  color: var(--courseo-slate-subtle);
}

.stage-tab.is-active .stage-tab__step {
  color: var(--courseo-tangerine);
}

.stage-tab__badge {
  font-size: 0.7rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  padding: 0.2rem 0.5rem;
  border-radius: 999px;
  background: var(--courseo-border);
  color: var(--courseo-obsidian);
}

.stage-tab__badge--accent {
  background: var(--courseo-tangerine-soft);
  color: var(--courseo-tangerine);
}

.stage-tab__title {
  font-size: 1rem;
  font-weight: 700;
  color: inherit;
}

.stage-tab__desc {
  font-size: 0.8125rem;
  color: var(--courseo-slate-muted);
}

.stage-tab__progress-track {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 3px;
  background: transparent;
}

.stage-tab__progress-fill {
  display: block;
  height: 100%;
  width: 0%;
  background: var(--courseo-tangerine);
  transition: width 0.08s linear;
}

/* Stage Viewport */
.hero-stage__viewport {
  padding: clamp(1.5rem, 3vw, 2.5rem);
}

.stage-panel {
  display: grid;
  grid-template-columns: minmax(0, 1.05fr) minmax(0, 1fr);
  gap: clamp(1.5rem, 3vw, 2.5rem);
  align-items: center;
  animation: courseoPanelIn 0.32s ease;
}

.stage-panel[hidden] {
  display: none;
}

@keyframes courseoPanelIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.stage-pill {
  display: inline-block;
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  padding: 0.3rem 0.7rem;
  border-radius: 999px;
  margin-bottom: 0.9rem;
}

.stage-pill--muted {
  background: var(--courseo-snow);
  border: 1px solid var(--courseo-border);
  color: var(--courseo-slate-muted);
}

.stage-pill--dark {
  background: var(--courseo-obsidian);
  color: var(--courseo-porcelain);
}

.stage-pill--accent {
  background: var(--courseo-tangerine-soft);
  color: var(--courseo-tangerine);
  border: 1px solid var(--courseo-tangerine-border);
}

.stage-panel__heading {
  font-size: clamp(1.45rem, 2.4vw, 2rem);
  line-height: 1.2;
  letter-spacing: -0.02em;
  margin: 0 0 0.85rem;
  color: var(--courseo-obsidian);
}

.stage-panel__copy {
  font-size: 0.975rem;
  line-height: 1.65;
  color: var(--courseo-slate-muted);
  margin: 0 0 1.35rem;
}

/* Skill Chips (Stage 2) */
.skill-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  list-style: none;
  padding: 0;
  margin: 0 0 1.35rem;
}

.skill-chip {
  font-size: 0.8rem;
  font-weight: 600;
  padding: 0.35rem 0.75rem;
  border-radius: var(--radius-sm);
  background: var(--courseo-snow);
  border: 1px solid var(--courseo-border);
  color: var(--courseo-obsidian);
}

/* Telemetry Cards */
.telemetry-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 0.75rem;
  margin: 0;
}

.telemetry-card {
  background: var(--courseo-snow);
  border: 1px solid var(--courseo-border);
  border-radius: var(--radius-md);
  padding: 0.9rem;
}

.telemetry-card--warn {
  border-color: #FCA5A5;
  background: #FEF2F2;
}

.telemetry-card--accent {
  background: var(--courseo-obsidian);
  border-color: var(--courseo-obsidian);
  color: var(--courseo-porcelain);
}

.telemetry-card dt {
  font-size: 0.72rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  color: var(--courseo-slate-muted);
  margin-bottom: 0.35rem;
}

.telemetry-card--accent dt {
  color: var(--courseo-slate-subtle);
}

.telemetry-card__value {
  margin: 0;
  font-family: var(--font-mono);
  font-size: 1.35rem;
  font-weight: 800;
  color: var(--courseo-obsidian);
}

.telemetry-card--accent .telemetry-card__value {
  color: var(--courseo-tangerine);
}

.telemetry-card__sub {
  display: block;
  margin-top: 0.2rem;
  font-size: 0.75rem;
  color: var(--courseo-slate-muted);
}

.telemetry-card--accent .telemetry-card__sub {
  color: #CBD5E1;
}

/* Artifact Windows (Code, Chat, Telemetry) */
.artifact-window {
  background: var(--courseo-obsidian);
  color: var(--courseo-porcelain);
  border-radius: var(--radius-md);
  border: 1px solid var(--courseo-obsidian-elevated);
  overflow: hidden;
  box-shadow: 0 18px 40px -18px rgba(15, 23, 42, 0.45);
}

.artifact-window--accent {
  border-color: var(--courseo-tangerine-border);
  box-shadow: 0 20px 44px -16px rgba(253, 105, 2, 0.22);
}

.artifact-window__bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  background: var(--courseo-obsidian-elevated);
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  font-family: var(--font-mono);
  font-size: 0.75rem;
}

.window-dots {
  display: inline-flex;
  gap: 6px;
}

.window-dots i {
  width: 9px;
  height: 9px;
  border-radius: 50%;
  background: #475569;
  display: block;
}

.artifact-window__title {
  color: #CBD5E1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.artifact-status {
  font-size: 0.68rem;
  font-weight: 700;
  padding: 0.18rem 0.5rem;
  border-radius: 999px;
  white-space: nowrap;
}

.artifact-status--idle {
  background: rgba(148, 163, 184, 0.18);
  color: #CBD5E1;
}

.artifact-status--active {
  background: rgba(253, 105, 2, 0.18);
  color: #FDBA74;
}

.artifact-status--live {
  background: var(--courseo-tangerine);
  color: var(--courseo-porcelain);
}

.artifact-window__body {
  padding: 1.15rem;
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
}

/* Stage 1 Chat Mockup */
.chat-bubble {
  padding: 0.85rem 1rem;
  border-radius: var(--radius-sm);
  font-size: 0.85rem;
  line-height: 1.5;
}

.chat-bubble--user {
  background: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.chat-bubble--bot {
  background: rgba(15, 23, 42, 0.65);
  border: 1px dashed rgba(148, 163, 184, 0.3);
  color: #CBD5E1;
}

.chat-bubble__role {
  display: block;
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--courseo-slate-subtle);
  margin-bottom: 0.3rem;
}

.chat-bubble p {
  margin: 0;
}

.artifact-footer-log {
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 0.5rem;
  font-family: var(--font-mono);
  font-size: 0.74rem;
  color: #FCA5A5;
  padding-top: 0.4rem;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
}

/* Stage 2 Code Block */
.code-block {
  margin: 0;
  padding: 1.15rem;
  overflow-x: auto;
  font-family: var(--font-mono);
  font-size: 0.8rem;
  line-height: 1.65;
  color: #E2E8F0;
}

.tok-kw { color: #FB923C; }
.tok-str { color: #86EFAC; }
.tok-cm { color: #64748B; }
.tok-num { color: #38BDF8; }

/* Stage 3 Production Repo Dashboard */
.repo-list {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.repo-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.7rem 0.85rem;
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.repo-row strong {
  display: block;
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: var(--courseo-porcelain);
}

.repo-row span {
  font-size: 0.74rem;
  color: var(--courseo-slate-subtle);
}

.repo-tag {
  font-family: var(--font-mono);
  font-size: 0.68rem !important;
  font-weight: 700;
  color: #4ADE80 !important;
  background: rgba(74, 222, 128, 0.12);
  padding: 0.22rem 0.55rem;
  border-radius: 999px;
}

.sandbox-bar {
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 0.5rem;
  padding: 0.7rem 0.85rem;
  border-radius: var(--radius-sm);
  background: var(--courseo-tangerine-soft);
  border: 1px solid var(--courseo-tangerine-border);
  font-family: var(--font-mono);
  font-size: 0.75rem;
  color: var(--courseo-porcelain);
}

.sandbox-bar__metric {
  color: #FDBA74;
  font-weight: 700;
}

/* Interactive Scrubber */
.hero-scrubber {
  padding: 1rem clamp(1.5rem, 3vw, 2.5rem) 1.35rem;
  background: var(--courseo-snow);
  border-top: 1px solid var(--courseo-border);
}

.hero-scrubber__labels {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--courseo-slate-muted);
  margin-bottom: 0.5rem;
}

.hero-scrubber__control {
  position: relative;
}

.hero-scrubber__input {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 8px;
  border-radius: 999px;
  background: linear-gradient(
    to right,
    var(--courseo-tangerine) 0%,
    var(--courseo-tangerine) var(--scrub-pct, 0%),
    var(--courseo-border) var(--scrub-pct, 0%),
    var(--courseo-border) 100%
  );
  outline: none;
  cursor: pointer;
}

.hero-scrubber__input::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--courseo-porcelain);
  border: 3px solid var(--courseo-tangerine);
  box-shadow: 0 2px 8px rgba(15, 23, 42, 0.2);
  cursor: grab;
}

.hero-scrubber__input::-moz-range-thumb {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--courseo-porcelain);
  border: 3px solid var(--courseo-tangerine);
  box-shadow: 0 2px 8px rgba(15, 23, 42, 0.2);
  cursor: grab;
}

/* Responsive */
@media (max-width: 920px) {
  .stage-panel {
    grid-template-columns: 1fr;
  }

  .hero-stage__tabs {
    grid-template-columns: 1fr;
  }

  .stage-tab {
    border-right: 0;
    border-bottom: 1px solid var(--courseo-border);
  }
}

@media (max-width: 560px) {
  .telemetry-grid {
    grid-template-columns: 1fr;
  }
}

@media (prefers-reduced-motion: reduce) {
  .stage-panel,
  .stage-tab__progress-fill,
  .courseo-btn {
    animation: none;
    transition: none;
  }
}
```

---

## C. Lightweight Vanilla JS Controller (`web/hero-story.js` or inline `<script>` in `web/index.html`)

Handles auto-play story progression (`requestAnimationFrame`), tab switching, live range-scrubber synchronization, pause-on-hover/interaction, keyboard navigation, and reduced-motion respect.

```javascript
(() => {
  const STAGE_DURATION_MS = 6500;
  const TOTAL_STAGES = 3;

  const root = document.getElementById("hero-transformation");
  const stageEl = document.getElementById("hero-story-stage");
  const scrubber = document.getElementById("hero-story-scrubber");
  const autoplayBtn = document.getElementById("hero-autoplay-toggle");
  const autoplayLabel = autoplayBtn?.querySelector("[data-autoplay-label]");

  if (!root || !stageEl || !scrubber) return;

  const tabs = Array.from(stageEl.querySelectorAll('[role="tab"]'));
  const panels = Array.from(stageEl.querySelectorAll('[role="tabpanel"]'));
  const progressBars = Array.from(stageEl.querySelectorAll("[data-progress-for]"));

  const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  let currentStage = 0;
  let stageElapsed = 0;
  let isAutoPlaying = !prefersReducedMotion;
  let isHovered = false;
  let lastTimestamp = null;
  let rafId = null;

  function updateScrubberVisual(percent) {
    const clamped = Math.max(0, Math.min(100, percent));
    scrubber.value = String(Math.round(clamped));
    scrubber.style.setProperty("--scrub-pct", `${clamped}%`);
  }

  function updateProgressBars(activeIndex, stageProgressRatio) {
    progressBars.forEach((bar, index) => {
      if (index < activeIndex) {
        bar.style.width = "100%";
      } else if (index === activeIndex) {
        bar.style.width = `${Math.round(stageProgressRatio * 100)}%`;
      } else {
        bar.style.width = "0%";
      }
    });
  }

  function activateStage(index, { resetProgress = true, syncScrubber = true } = {}) {
    const nextIndex = Math.max(0, Math.min(TOTAL_STAGES - 1, index));
    currentStage = nextIndex;
    root.dataset.stage = String(nextIndex);

    if (resetProgress) {
      stageElapsed = 0;
    }

    tabs.forEach((tab, i) => {
      const active = i === nextIndex;
      tab.classList.toggle("is-active", active);
      tab.setAttribute("aria-selected", String(active));
      tab.tabIndex = active ? 0 : -1;
    });

    panels.forEach((panel, i) => {
      const active = i === nextIndex;
      panel.classList.toggle("is-active", active);
      panel.hidden = !active;
    });

    const ratio = stageElapsed / STAGE_DURATION_MS;
    updateProgressBars(nextIndex, isAutoPlaying ? ratio : 1);

    if (syncScrubber) {
      const overallPct = ((nextIndex + (isAutoPlaying ? ratio : 0)) / TOTAL_STAGES) * 100;
      updateScrubberVisual(nextIndex === TOTAL_STAGES - 1 && !isAutoPlaying ? 100 : overallPct);
    }
  }

  function setAutoPlay(enabled) {
    isAutoPlaying = enabled;
    lastTimestamp = null;

    if (autoplayBtn) {
      autoplayBtn.setAttribute("aria-pressed", String(enabled));
    }
    if (autoplayLabel) {
      autoplayLabel.textContent = enabled ? "Pause Story" : "Play Story";
    }

    if (enabled) {
      startLoop();
    } else {
      stopLoop();
      updateProgressBars(currentStage, 1);
    }
  }

  function tick(timestamp) {
    if (!isAutoPlaying) return;

    if (lastTimestamp === null) {
      lastTimestamp = timestamp;
    }

    if (!isHovered) {
      const delta = timestamp - lastTimestamp;
      stageElapsed += delta;

      if (stageElapsed >= STAGE_DURATION_MS) {
        const nextStage = (currentStage + 1) % TOTAL_STAGES;
        activateStage(nextStage, { resetProgress: true, syncScrubber: true });
      } else {
        const stageRatio = stageElapsed / STAGE_DURATION_MS;
        updateProgressBars(currentStage, stageRatio);
        const overallPct = ((currentStage + stageRatio) / TOTAL_STAGES) * 100;
        updateScrubberVisual(overallPct);
      }
    }

    lastTimestamp = timestamp;
    rafId = window.requestAnimationFrame(tick);
  }

  function startLoop() {
    if (rafId === null) {
      rafId = window.requestAnimationFrame(tick);
    }
  }

  function stopLoop() {
    if (rafId !== null) {
      window.cancelAnimationFrame(rafId);
      rafId = null;
    }
  }

  // Tab Click & Keyboard Navigation
  tabs.forEach((tab, index) => {
    tab.addEventListener("click", () => {
      activateStage(index, { resetProgress: true, syncScrubber: true });
    });

    tab.addEventListener("keydown", (event) => {
      if (event.key === "ArrowRight") {
        event.preventDefault();
        const next = (index + 1) % TOTAL_STAGES;
        tabs[next].focus();
        activateStage(next, { resetProgress: true, syncScrubber: true });
      } else if (event.key === "ArrowLeft") {
        event.preventDefault();
        const prev = (index - 1 + TOTAL_STAGES) % TOTAL_STAGES;
        tabs[prev].focus();
        activateStage(prev, { resetProgress: true, syncScrubber: true });
      }
    });
  });

  // Interactive Range Scrubber
  scrubber.addEventListener("input", (event) => {
    const val = Number(event.target.value);
    if (isAutoPlaying) {
      setAutoPlay(false);
    }

    const mappedStage = val < 34 ? 0 : val < 67 ? 1 : 2;
    if (mappedStage !== currentStage) {
      activateStage(mappedStage, { resetProgress: true, syncScrubber: false });
    }
    updateScrubberVisual(val);
    updateProgressBars(mappedStage, 1);
  });

  // Auto-play Toggle Button
  autoplayBtn?.addEventListener("click", () => {
    setAutoPlay(!isAutoPlaying);
  });

  // Pause timer smoothly while user inspects code/telemetry
  stageEl.addEventListener("mouseenter", () => {
    isHovered = true;
  });

  stageEl.addEventListener("mouseleave", () => {
    isHovered = false;
    lastTimestamp = null;
  });

  // Initialize
  activateStage(0, { resetProgress: true, syncScrubber: true });
  setAutoPlay(isAutoPlaying);
})();
```

Status: success (code: 0) | Tokens: 760 | Cost: $0.0001 | Artifacts: 1
