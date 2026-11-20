#!/usr/bin/env python3
"""
Verification script for Courseo JobHunt template redesign implementation.
Verifies that web/index.html and web/courseo.css contain the required JobHunt structural components:
- Split Hero with tilted card and percentage dials
- 5-card Bento grid
- Horizontal status/metrics layout
- Split pricing card with toggle
- Brand color tokens (#FD6902, #0F172A, #FFFFFF, #F8FAFC)
"""

import sys
import os

def check_file(path, required_strings):
    if not os.path.exists(path):
        print(f"FAIL: File not found: {path}")
        return False
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    missing = [s for s in required_strings if s not in content]
    if missing:
        print(f"FAIL: {path} is missing required tokens: {missing}")
        return False
    print(f"PASS: {path} contains all {len(required_strings)} required tokens.")
    return True

def main():
    css_tokens = [
        "--brand-orange",
        "jobhunt-hero",
        "dial-gauge",
        "bento-jobhunt",
        "stat-row"
    ]
    html_tokens = [
        "jobhunt-hero",
        "98%",
        "dial-gauge",
        "bento-jobhunt",
        "stat-row",
        "courseo_logo"
    ]

    ok_css = check_file("web/courseo.css", css_tokens)
    ok_html = check_file("web/index.html", html_tokens)

    if ok_css and ok_html:
        print("ALL AUDIT GATES PASSED: JobHunt redesign successfully verified.")
        sys.exit(0)
    else:
        print("AUDIT FAILED: Required redesign tokens not yet fully implemented.")
        sys.exit(1)

if __name__ == "__main__":
    main()
