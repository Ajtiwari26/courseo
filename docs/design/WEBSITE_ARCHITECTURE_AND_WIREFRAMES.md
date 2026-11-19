# Courseo Website Architecture & UX Wireframes Specification
## Document ID: ARCH-COURSEO-WEB-2026-002
**Product:** Courseo (Next-Generation EdTech & Premium Video Course Marketplace)  
**Target:** `/Users/ajaytiwari/Desktop/Projects/courseo/docs/design/WEBSITE_ARCHITECTURE_AND_WIREFRAMES.md`

---

## 1. Information Architecture & Sitemap

```
                                  [ COURSEO PLATFORM ]
                                           │
         ┌─────────────────────────────────┼─────────────────────────────────┐
         ▼                                 ▼                                 ▼
[ 1. Marketplace Front ]         [ 2. Student Experience ]         [ 3. Creator Studio ]
   ├── / (Landing Page)             ├── /dashboard (My Courses)       ├── /studio/courses (Drafts)
   ├── /courses (Catalog)           ├── /learn/:courseId/play         ├── /studio/drm-telemetry
   ├── /course/:slug (Detail)       │   └── SecureWatch Player        └── /studio/earnings
   ├── /instructors (Faculty)       ├── /cert/:id (Verification)
   └── /pricing (Bundles)           └── /account/settings
```

---

## 2. Master Landing Page Wireframe & Layout

### 2.1 Section 1: Floating Glass Navigation Bar (`Header`)
* **Styling**: `position: sticky; top: 16px; backdrop-filter: blur(16px); background: rgba(14, 16, 22, 0.82); border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 9999px;`
* **Elements**:
  - Left: Courseo Logo (3D Hexagon Orange Ribbon icon + `cours`**`eo`** brand lockup).
  - Center: Nav Links (`Explore Courses`, `Curriculum`, `Enterprise`, `Creators`) with smooth hover underlines.
  - Right: Search input with `⌘K` keyboard shortcut pill + "Sign In" link + Primary CTA button ("Get Started" in Radiant `#FD6902`).

---

### 2.2 Section 2: Cinema-Grade Hero with Embedded Live Player
* **Headline**: "Master the Frontier of Tech, Code & Design."
* **Subheadline**: "Learn from the top 1% of practitioners through high-production video courses—streamed in zero-latency 4K with next-generation SecureWatch content protection."
* **Action Row**:
  - Primary CTA: "Browse All Courses" (Radiant orange with hover glow).
  - Secondary CTA: "Watch Platform Reel" (Ghost button with play icon).
* **Focal Anchor: Interactive SecureWatch Hero Compositor**:
  - A real embedded interactive player running the WebGL anti-camcording shader and dynamic drifting watermark.
  - Overlay HUD badge: `● PROTECTED LIVE FEED` in luminous emerald `#10B981`.
  - Floating pill tags around the player: `4K 60fps`, `Zero-Buffering`, `Forensically Attributed`.

---

### 2.3 Section 3: Social Proof & Metrics Strip
* Horizontal grid highlighting credibility:
  - **48,000+** Active Students Enrolled
  - **94.6%** Verified Course Completion Rate
  - **120+** Elite Masterclasses Published
  - **$0** Piracy Leak Incident Rate (Secured by Courseo SecureWatch)

---

### 2.4 Section 4: Feature Bento Grid (Why Courseo Wins)

```
+-------------------------------------------------------+-----------------------------------+
| CARD 1 (Spans 2 Columns):                             | CARD 2 (Spans 1 Column):          |
| Cinema-Grade Streaming & Zero-Leak Security           | Real-World Hands-On Sandboxes     |
| - WebGL anti-camcording modulation                    | - In-browser execution            |
| - Forensic BCH & DCT leak watermarking                | - Integrated code challenges      |
| - Hardware EME & offline desktop app                  | - Real-time automated grading     |
+-----------------------------------+-------------------+-----------------------------------+
| CARD 3 (Spans 1 Column):          | CARD 4 (Spans 2 Columns):                             |
| 1-on-1 Creator Office Hours       | Cryptographically Verified Certificates           |
| - Private cohort channels         | - Verifiable credentials on public ledger         |
| - Weekly live video Q&A           | - 1-click LinkedIn export and portfolio embedding |
+-----------------------------------+-------------------------------------------------------+
```

---

### 2.5 Section 5: Filterable Course Catalog Grid
* **Category Tabs**: `All`, `Systems & Rust`, `AI & Machine Learning`, `UI/UX & Product`, `Distributed Systems`, `Founders`.
* **Course Card Anatomy**:
  - Aspect Ratio `16:9` video thumbnail with 3-second animated preview reel on hover.
  - Level pill (`Beginner`, `Intermediate`, `Advanced`).
  - Duration & Lesson count (`8.5 hours • 34 lessons`).
  - Course Title (`e.g., "High-Performance Systems in Rust & GPU Shaders"`).
  - Instructor Avatar, Name, and Role (`e.g., "Elena Rostova, Principal Systems Architect"`).
  - Star rating (`4.9 ★ (1,420 reviews)`).
  - Price & Enroll CTA button (`$149 • Enroll Now`).

---

### 2.6 Section 6: High-Conversion Conversion Footer
* **Radiant Glow Ambient Atmosphere**: Subtle deep orange radial backlighting.
* **Punchy Closing Hook**: "Ready to Accelerate Your Career? Join 48,000+ Professionals Today."
* **Action**: Email capture with instant enrollment preview + Guarantee badge ("30-Day Money-Back Guarantee • Lifetime Access").
