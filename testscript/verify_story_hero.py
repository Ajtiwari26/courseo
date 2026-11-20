#!/usr/bin/env python3
"""
Verification script for Courseo Hero Pinned Scroll Reveal, 3D Assets,
Faculty Avatars, and Cohort Showreel Modal integration.
"""

import sys
import os

def check_file(path, tokens):
    if not os.path.exists(path):
        print(f"FAIL: File not found: {path}")
        return False
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    missing = [t for t in tokens if t not in content]
    if missing:
        print(f"FAIL: {path} is missing tokens: {missing}")
        return False
    print(f"PASS: {path} contains all {len(tokens)} tokens.")
    return True

def main():
    html_tokens = [
        "hero-scroll-story",
        "scroll-story",
        "scroll-step",
        "cockpit-stage",
        "assets/hero/stage-1-prompt-plateau",
        "assets/hero/stage-2-deep-skills-leap",
        "assets/hero/stage-3-10x-superpower",
        "assets/tracks/track-agent-swarms.jpg",
        "assets/tracks/track-generative-cinema.jpg",
        "assets/tracks/track-local-llm.jpg",
        "assets/tracks/track-algorithmic-growth.jpg",
        "openShowreelBtn",
        "showreelModal",
        "showreelVideo",
        "assets/faculty/rohan_verma.svg",
        "assets/faculty/ananya_nair.svg",
        "assets/faculty/kabir_mehta.svg",
        "assets/faculty/devika_sharma.svg",
        "assets/faculty/vikramaditya_sen.svg"
    ]
    css_tokens = [
        "scroll-story",
        "scroll-story__sticky",
        "cockpit-stage",
        "scroll-step",
        "scroll-story__progress",
        "scroll-story__dock",
        "bento-card__media",
        "faculty-card__media",
        "showreel-modal"
    ]
    js_tokens = [
        "hero-scroll-story",
        "updateOnScroll",
        "setStage",
        "openShowreel",
        "closeShowreel"
    ]

    physical_assets = [
        "web/assets/hero/stage-1-prompt-plateau.jpg",
        "web/assets/hero/stage-2-deep-skills-leap.jpg",
        "web/assets/hero/stage-3-10x-superpower.jpg",
        "web/assets/tracks/track-agent-swarms.jpg",
        "web/assets/tracks/track-generative-cinema.jpg",
        "web/assets/tracks/track-local-llm.jpg",
        "web/assets/tracks/track-algorithmic-growth.jpg",
        "web/assets/faculty/rohan_verma.svg",
        "web/assets/faculty/ananya_nair.svg",
        "web/assets/faculty/kabir_mehta.svg",
        "web/assets/faculty/devika_sharma.svg",
        "web/assets/faculty/vikramaditya_sen.svg",
        "web/assets/courseo_motion_reveal.mp4",
        "web/assets/brand/motion_frame_05.jpg"
    ]

    missing_assets = [p for p in physical_assets if not os.path.exists(p)]
    if missing_assets:
        print(f"FAIL: Missing physical asset files: {missing_assets}")
        sys.exit(1)
    print(f"PASS: All {len(physical_assets)} physical media assets verified on disk.")

    ok_html = check_file("web/index.html", html_tokens)
    ok_css = check_file("web/courseo.css", css_tokens)
    ok_js = check_file("web/hero-story.js", js_tokens)

    if ok_html and ok_css and ok_js:
        print("ALL AUDIT GATES PASSED: Pinned Scroll Reveal, 3D Assets, and Modal successfully verified.")
        sys.exit(0)
    else:
        print("AUDIT FAILED: Required tokens missing.")
        sys.exit(1)

if __name__ == "__main__":
    main()
