File version: 1.01

**TLDR:**
This document lists the core application requirements for `gfx-test`:
* Core application features
* Video debug output feature
* Debug output activated by `debug-video-to-png` feature flag

# Core Application Requirements

- The application SHALL display a fullscreen window.
- The application SHALL render pixels (multiple boxes) on the screen.
- The application SHALL animate the rendered pixels.
- The application SHALL close when the 'Escape' key is pressed.

# Video Debug Output Requirements

- The application SHALL allocate 10 frame buffers at program startup for
  storing screen content.
- The application SHALL copy the screen canvas content to a frame buffer at
  `frame_counter` 1, 100, 200, 300, 400, 500, 600, 700, 800, and 900.
- The application SHALL save the 10 captured frame buffers to PNG files upon
  program exit (e.g., when the 'Escape' key is pressed).
- The saved PNG files SHALL be named descriptively (e.g.,
  `output/frame_0001.png`, `output/frame_0100.png`).
- The purpose of these PNG files is to provide visual test output for
  debugging and verification.
- The video debug output SHALL be activated by a feature flag named
  `debug-video-to-png`.
