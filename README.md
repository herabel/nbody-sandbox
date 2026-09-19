# nbody-sandbox

<img width="2560" height="1369" alt="image" src="https://github.com/user-attachments/assets/abb6bfb3-cb2f-4772-aa01-29f0bc27eb35" />


An experimental real-time 2D N-body gravitational physics simulator written in Rust, built with [macroquad](https://github.com/not-fl3/macroquad) and [egui](https://github.com/emilk/egui).

Simulates celestial mechanics with numerical stability, high frame rates (150-200+ FPS on 700+ bodies), interactive camera controls, and configurable telemetry HUD.

---

## Features

- **Symplectic Leapfrog Integration (Kick-Drift-Kick):**
  Second-order time-reversible numerical integrator preserving the Hamiltonian invariant (total energy and phase space volume). Orbits remain stable over thousands of cycles without artificial energy drift.
- **Gravitational Softening ($\epsilon$):**
  Implements a Plummer-like softening factor avoiding numerical singularities ($r \to 0$) and artificial kinetic ejections during close perihelion passes.
- **Zero-Allocation Hot Path:**
  Rendering and telemetry formatting reuse persistent pre-allocated buffers (`String::with_capacity` + `.clear()`), eliminating allocator lock contention and maintaining deterministic frame times.
- **Interactive UI (egui) & HUD:**
  Real-time control over coordinate grids, telemetry readouts (position, velocity vector, acceleration vector, mass), font sizes, and visual guide lines.
- **2D Camera System:**
  Viewport zooming (mouse wheel) and navigation across multiple astronomical scales (from inner planet orbits to distant Kuiper belt bodies).

---

## Presets

- **Solar System:**
  Modeled with authentic relative mass ratios and astronomical unit (AU) scalings:
- Central Sun ($M = 330{,}000$)
- Terrestrial planets: Mercury, Venus, Earth, Mars
- Asteroid belt: Ceres, Vesta, Pallas
- Gas & ice giants: Jupiter, Saturn, Uranus, Neptune
- Kuiper belt: Pluto
- **Galaxy Accretion Disk:**
  Procedural generator spawning 700+ bodies in stable differential Keplerian rotation around a supermassive central core.

---

## Controls

| Action | Control |
|---|---|
| **Zoom In / Out** | Mouse Wheel |
| **Telemetry HUD** | Toggle checkboxes in the Menu window |
| **Grid Overlay** | Toggle and adjust thickness / spacing in Menu |

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable, 2021 edition)

### Build & Run
```bash
cargo run --release
```