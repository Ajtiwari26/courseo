# Courseo Master Platform Architecture: Next-Gen AI Curriculum, Cinematic Scroll Experience & WebOrient Voice Agent Integration
## Document ID: SPEC-COURSEO-CURRICULUM-AND-SCROLL-2026-003
**Classification:** Product Master Plan & Interactive Design Specification  
**Brand Identity:** Courseo (`#FD6902` Radiant Tangerine + Cinema Obsidian `#0A0B0E`)  
**Target:** `/Users/ajaytiwari/Desktop/Projects/courseo/docs/design/COURSEO_EXPANDED_CURRICULUM_AND_SCROLL_EXPERIENCE.md`

---

## 1. Executive Vision: The Ultimate AI & Creator Masterclass Platform

Courseo is positioned as the definitive educational marketplace for the AI revolution, bringing together elite software engineering, autonomous agents, generative filmmaking, and performance monetization under a single cinema-grade platform. 

Every course is delivered through the proprietary **Courseo SecureWatch** WebGL engine (zero-leak, camcorder-resistant DRM) and accompanied by **WebOrient**, an embedded multimodal voice co-browsing agent with an on-screen Ghost Cursor that converses with prospective students, navigates the curriculum, and provides personalized course recommendations.

---

## 2. Expanded Curriculum Architecture (3 Core Tracks & 12 Disciplines)

```
====================================================================================================
                                      COURSEO CURRICULUM UNIVERSE
====================================================================================================
[ TRACK 1: AI ENGINEERING & AUTONOMOUS SYSTEMS ]
  ├── 1. Web Development via AI (Cursor, Claude Code, Windsurf, v0, Supabase, Next.js 15)
  ├── 2. Autonomous AI Agents & Swarms (OpenClaw, Hermes, LangGraph, AGY/Etta, Multi-Agent DAGs)
  ├── 3. Next-Gen Voice Agents (Gemini Live API, Cartesia, ElevenLabs, WebSockets, Low-Latency VAD)
  ├── 4. Local AI Inference & SLM Training (Llama 3.3, DeepSeek R1/V3, Qwen 2.5, Ollama, vLLM, Unsloth LoRA)
  ├── 5. AI Automation for Enterprise (n8n, Autonomous Browser Agents, Python Copilots, API Orchestration)
  └── 6. Mobile App Development with AI (Cross-platform Flutter & React Native with AI Pair-Programmers)

[ TRACK 2: GENERATIVE AI FILMMAKING & CREATIVE MEDIA ]
  ├── 7. AI Filmmaking & Cinematography (Higgsfield, Seedance 2.5, Runway Gen-3, Kling, Midjourney/Flux)
  ├── 8. Video Ads Generation & High-Converting UGC (AI Avatars, Viral Hooks, E-commerce Creative Sprays)
  ├── 9. Motion Graphics & Cinema Post-Production (Premiere Pro, After Effects, Kinetic Type, Generative FX)
  └── 10. AI Music Production & Sound Design (Suno v4, Udio, Foley Synthesis, Commercial Audio Mastering)

[ TRACK 3: HIGH-ROI GROWTH, COMMERCE & AD SCALING ]
  ├── 11. Meta Ads Scaling Mastery (Facebook/Instagram Performance Marketing, Advantage+, Creative Testing)
  └── 12. E-Commerce Brand Building (Shopify Store Design, High-Converting Landing Funnels, CRO & AOV)
====================================================================================================
```

### Detailed Breakdown of Flagship Masterclasses

| Track | Course Title | Lead Instructor Persona | Key Tech & Deliverables |
|---|---|---|---|
| **AI Eng** | *Building Autonomous Multi-Agent Swarms with OpenClaw & Hermes* | Principal AI Architect (Ex-DeepMind) | LangGraph, OpenClaw, Etta DAGs, Sandboxed Execution |
| **AI Eng** | *Real-Time Conversational Voice Agents with Gemini Live & WebSockets* | Voice AI Systems Lead | Gemini 3.8/Live, Cartesia, Bidirectional Audio, Tool Calling |
| **AI Eng** | *Zero-to-One Local LLM Inference & LoRA Fine-Tuning* | Open-Source ML Engineer | DeepSeek R1, vLLM, SGLang, Ollama, Unsloth, Quantization |
| **Creative** | *AI Cinema & Virtual Production: The Higgsfield & Seedance 2.5 Guide* | Award-Winning AI Cinematographer | Seedance 2.5, Higgsfield, Runway Gen-3, Multi-Camera Prompts |
| **Creative** | *Viral AI Video Ads: E-Commerce Creative Testing Pipeline* | 8-Figure DTC Creative Director | AI UGC Avatars, CapCut AI, 3-Second Retention Hooks, A/B Splits |
| **Growth** | *Meta Ads 2026/2027: Scaling DTC to $1M/Month with Machine Learning* | Performance Marketing Partner | Advantage+ Shopping, Conversion API, Creative Fatigue Cycles |

---

## 3. Cinematic Scroll-Reveal Interaction Model & Layout

The Courseo landing page uses a **progressive narrative scroll experience** modeled after Awwwards-winning benchmarks (Apple, Linear, Whop, Curated.design) combining:
- **Lenis Physics**: Smooth momentum-based scrolling with zero input jitter.
- **Native CSS `view-timeline`**: GPU-composited, zero-JS entry reveals for high framerate on all devices.
- **GSAP ScrollTrigger**: Section pinning, sticky card stacking, and horizontal curriculum carousels.

```
+----------------------------------------------------------------------------------------------------+
|                                      SCROLL JOURNEY TIMELINE                                       |
+----------------------------------------------------------------------------------------------------+
| [STAGE 0: HERO]                                                                                    |
|  - Floating Glass Navigation Bar (Frosted blur 24px, logo with orange audio chime)                 |
|  - Split-Text Headline Reveal: "Master the Frontier of Tech, Code & Design."                       |
|  - Embedded Live SecureWatch WebGL Player with interactive anti-camcording HUD badge               |
|                                                                                                    |
| [STAGE 1: THE REVEAL CRUISE]                                                                       |
|  - Metrics counter triggers on entry: 48,000+ Enrolled • 94.6% Completion Rate • $0 Piracy        |
|  - Ambient orange gradient flare drifts downward following the scroll momentum                    |
|                                                                                                    |
| [STAGE 2: CURRICULUM MATRIX (HORIZONTAL PINNED TRACK)]                                             |
|  - Section pins in place while user scrolls horizontally through the 3 Core Tracks                 |
|  - Interactive Track Selector Tabs with active glowing slider indicator                            |
|  - Course Cards with 3-Second Hover Video Scrub and Level Badges                                   |
|                                                                                                    |
| [STAGE 3: THE BENTO GRID (STICKY STACKING CARDS)]                                                  |
|  - Card 1: Courseo SecureWatch Engine (WebGL rolling shutter anti-camcording demo)                 |
|  - Card 2: Interactive In-Browser Coding & Sandbox Terminals                                       |
|  - Card 3: WebOrient Voice Co-Browsing Copilot (Watch Ghost Cursor navigate the UI live)           |
|  - Card 4: Cryptographically Verified On-Chain & LinkedIn Certifications                           |
|                                                                                                    |
| [STAGE 4: CREATOR & FACULTY SPOTLIGHT]                                                             |
|  - Staggered portrait cards with audio quote snippets and credentials                              |
|                                                                                                    |
| [STAGE 5: RADIANT CONVERSION FOOTER]                                                               |
|  - Pulsing orange backlight flare: "Your Future in AI Starts Today"                                |
|  - 1-Click Enrollment & Lifetime Pass Checkout with 30-Day Money-Back Guarantee                    |
+----------------------------------------------------------------------------------------------------+
```

---

## 4. WebOrient Voice Co-Browsing Agent Integration Specification

### 4.1 What WebOrient Does on Courseo
Located at `/Users/ajaytiwari/Desktop/Projects/weborient`, WebOrient is a **voice-to-voice multimodal AI companion** with an interactive on-screen **Ghost Cursor**. On Courseo, it serves as an instant conversational educational advisor:

```
[ Prospective Student ]
        │ (Speaks: "Show me courses on autonomous agents and local models")
        ▼
[ WebOrient Live Client (WebSockets / Web Audio 16kHz PCM) ]
        │
        ├──> Streams speech to Gemini Multimodal Live API
        │
        └──> Dispatches Tool Call: navigate_to_curriculum(track="ai-agents")
                │
                ▼
[ Branded Ghost Cursor (Tangerine #FD6902 with Luminous Trail) ]
        │
        ├── Moves via smooth cubic-Bézier curves to "Autonomous AI Agents" card
        ├── Smoothly scrolls page into viewport
        ├── Triggers illuminated radiant spotlight border around the card
        └── Speaks: "Here is our flagship masterclass on OpenClaw and Hermes. 
                     Would you like me to open the syllabus preview?"
```

### 4.2 Drop-in Integration Schema for Courseo
```html
<!-- Courseo WebOrient Integration Script -->
<script 
  src="/sdk/weborient.js"
  data-endpoint="https://api.courseo.io/v1/live-relay"
  data-theme-color="#FD6902"
  data-cursor-style="modern-glow"
  data-manifest="/weborient-courseo-manifest.json"
  data-agent-name="Courseo Guide"
  data-greeting="Hi! I am your Courseo Guide. Ask me anything about our AI, filmmaking, or marketing courses!">
</script>
```

### 4.3 WebOrient S-Tier UI Presentation
- **Floating Audio Orb / Pill (Bottom-Right)**:
  - Frosted glass container with dynamic audio waveform bars pulsing in `#FD6902`.
  - Label: `🎙️ Ask Courseo AI • Click or Space to Speak`.
  - Tooltip: `Try: "Compare AI Agents vs Web Dev" or "Take me to Meta Ads"`.

---

## 5. Visual Styling, Glass Theme & Component Tokens

### 5.1 Glass Theme Tokens
```css
:root {
  /* Surface Layers */
  --bg-deep-obsidian: #0A0B0E;
  --bg-card-glass: rgba(20, 22, 28, 0.72);
  --bg-card-elevated: rgba(28, 30, 38, 0.85);
  
  /* Brand Radiant Accents */
  --accent-orange: #FD6902;
  --accent-orange-glow: rgba(253, 105, 2, 0.35);
  --accent-orange-subtle: rgba(253, 105, 2, 0.12);
  
  /* Strokes & Highlights */
  --border-glass-hairline: 1px solid rgba(255, 255, 255, 0.08);
  --border-glass-glow: 1px solid rgba(253, 105, 2, 0.40);
  --specular-highlight: inset 0 1px 0 rgba(255, 255, 255, 0.12);
  
  /* Depth Shadows */
  --shadow-bento: 0 20px 50px rgba(0, 0, 0, 0.65), var(--specular-highlight);
  --shadow-glow: 0 0 35px var(--accent-orange-glow);
}
```

### 5.2 Icon & Visual Assets Policy
- Strictly SVG iconography from **Lucide** and **Hugeicons** (2px stroke width, monochrome with orange accent hovers). Zero generic emojis.
- High-contrast typography pairing: `Plus Jakarta Sans` (Display/Headings) + `Inter` (UI/Body) + `JetBrains Mono` (Code & Security).

---

## 6. Pre-Dispatch Package for Gemini 4 Neon (`--model neon`)

This research document equips Neon with complete domain grounding across:
1. Exact course titles, instructors, and categorization for the 12 masterclasses.
2. Complete scroll-reveal timeline (Lenis + CSS `view-timeline` + GSAP ScrollTrigger).
3. Technical contract for embedding the **WebOrient Ghost Cursor Agent** with the `#FD6902` brand theme.
4. Glassmorphic component styles and anti-AI-slop design system.
