# Courseo Brand Identity & Design System Specification (v1.0)
## Document ID: DESIGN-COURSEO-2026-001
**Classification:** Brand Identity, Aesthetic Philosophy, and Component Design System  
**Product:** Courseo (Next-Generation EdTech & Premium Video Course Marketplace)  
**Target:** `/Users/ajaytiwari/Desktop/Projects/courseo/docs/design/COURSEO_BRAND_IDENTITY_AND_DESIGN_SYSTEM.md`

---

## 1. Brand Essence & Visual Identity

### 1.1 The Courseo Narrative
Courseo is an elite, modern educational technology marketplace where world-class creators and industry experts publish and sell premium video courses. Unlike traditional platforms (Udemy, Coursera) that suffer from dated layouts and rampant course piracy, Courseo combines:
1. **Cinema-Grade Aesthetic**: Dark, immersive, hyper-polished visual language inspired by modern creative platforms (Linear, Whop, MasterClass, Reforge).
2. **Proprietary Video Security (Courseo SecureWatch)**: Zero-leak, camcorder-resistant DRM with forensic leak attribution baked into an elegant WebGL player.
3. **Anti-AI-Slop Craftsmanship**: Grounded in the design philosophy of `superdesigndev/superdesign-skill`—refusing generic template cards, flat bootstrap grids, and tasteless AI-generated clichés in favor of bespoke typography, purposeful Bento layouts, tactile micro-interactions, and refined color harmony.

---

## 2. Brand Asset Inventory & Extraction

The official brand assets have been cataloged and permanently stored under [`assets/brand/`](file:///Users/ajaytiwari/Desktop/Projects/courseo/assets/brand/):

| Asset Filename | Format | Description & Usage |
|---|---|---|
| [`courseo_logo_primary.jpg`](file:///Users/ajaytiwari/Desktop/Projects/courseo/assets/brand/courseo_logo_primary.jpg) | High-Res Raster (1024x1024) | Primary lockup: Hexagonal Orange 3D Ribbon "C" + inset Play glyph + `cours` (Charcoal) `eo` (Orange) wordmark |
| [`courseo_logo_reveal_motion.mp4`](file:///Users/ajaytiwari/Desktop/Projects/courseo/assets/brand/courseo_logo_reveal_motion.mp4) | H.264 1080p/720p 24fps (10.0s) | Official motion graphics identity reveal with ambient chime soundscape |
| [`design_inspiration_resources_1.jpg`](file:///Users/ajaytiwari/Desktop/Projects/courseo/assets/brand/design_inspiration_resources_1.jpg) | Reference Snapshot | Curated list (`jiro.build`, `land-book.com`, `recent.design`, `landdding.com`, `fontinlogo.com`) |
| [`design_inspiration_resources_2.jpg`](file:///Users/ajaytiwari/Desktop/Projects/courseo/assets/brand/design_inspiration_resources_2.jpg) | Reference Snapshot | `superdesign-skill` anti-AI-slop philosophy & inspiration catalogs (`curated.design`, `landing.love`, `saaspo.com`, `component.gallery`) |

### 2.1 Color Palette Specifications

Extracted via direct image spectral quantization from [`courseo_logo_primary.jpg`](file:///Users/ajaytiwari/Desktop/Projects/courseo/assets/brand/courseo_logo_primary.jpg):

```
+-----------------------------------------------------------------------------------------+
|                                  COURSEO COLOR TOKENS                                   |
+-----------------------------------------------------------------------------------------+
| [Primary Brand Accent]  : #FD6902 (RGB: 253, 105, 2)  - Courseo Electric Orange        |
| [Primary Hover/Glow]    : #FF8533 (RGB: 255, 133, 51) - Luminous Tangerine Radiant     |
| [Subtle Accent Surface] : rgba(253, 105, 2, 0.12)     - Soft Amber Tint for Badges     |
| [Deep Foundation Dark]  : #0A0B0E (RGB: 10, 11, 14)   - Cinema Obsidian Black          |
| [Card / Surface Layer]  : #14161C (RGB: 20, 22, 28)   - Elevated Matte Graphite        |
| [Card Border / Stroke]  : rgba(255, 255, 255, 0.08)   - Frosted Glass Border Subtlety  |
| [Text High Contrast]    : #FFFFFF (RGB: 255, 255, 255)- Pure Optical Heading White     |
| [Text Muted / Body]     : #94A3B8 (RGB: 148, 163, 184)- Slate Subdued Readability      |
| [Success Green]         : #10B981 (RGB: 16, 185, 129) - Verified Badge / Enrolled      |
+-----------------------------------------------------------------------------------------+
```

---

## 3. Typography Hierarchy & Rules

* **Display & Heading Typeface**: `Plus Jakarta Sans` or `Cabinet Grotesk` (geometric, modern, authoritative with humanistic warmth).
* **Body & Interface Typeface**: `Inter` (neutral, crisp x-height, flawless legibility across small labels, timestamps, and curriculum metadata).
* **Monospace / Code / Security Tags**: `JetBrains Mono` or `Fira Code` (for DRM telemetry, session tokens, lesson timestamps, and coding exercises).

```css
/* Typography Scale */
--font-display: 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, sans-serif;
--font-body: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
--font-mono: 'JetBrains Mono', monospace;

/* Type Sizes */
--text-hero: clamp(2.5rem, 6vw, 4.25rem); /* 40px to 68px */
--text-h1: clamp(2rem, 4vw, 3rem);        /* 32px to 48px */
--text-h2: clamp(1.5rem, 3vw, 2.25rem);   /* 24px to 36px */
--text-h3: 1.25rem;                       /* 20px */
--text-body: 1rem;                        /* 16px */
--text-sm: 0.875rem;                      /* 14px */
--text-xs: 0.75rem;                       /* 12px */
```

---

## 4. Design Tenets & Anti-AI-Slop Rules

1. **Depth Through Lighting, Not Clutter**:
   - Dark surfaces are never pure `#000000` flat planes. They use subtle radial mesh gradients (orange ambient glow `rgba(253, 105, 2, 0.06)` emanating from behind key focal points).
2. **Tactile Bento Grid Layout**:
   - Content and platform features are presented in asymmetrical, highly scannable Bento cards with rounded corners (`16px` to `24px`), hairline borders (`1px solid rgba(255,255,255,0.08)`), and soft hover elevation.
3. **Interactive Player Showcasing**:
   - Rather than static screenshot mockups, the hero section embeds an **active, live WebGL Courseo SecureWatch Player preview** where visitors can toggle anti-camcording modulation, preview the forensic drifting watermark, and test canvas security traps.
4. **Micro-Interactions & Motion Rhythm**:
   - Buttons have a 150ms cubic-bezier transition with a luminous orange perimeter glow on hover.
   - Course cards feature video hover scrub (pre-loading a 3-second preview reel upon hovering).
