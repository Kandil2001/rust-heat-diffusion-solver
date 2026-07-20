# 2D Finite Volume Heat Conduction Solver in Rust

<p align="center">
  <img src="https://img.shields.io/badge/Status-Completed-brightgreen.svg" alt="Completed">
  <a href="https://github.com/Kandil2001/Rust-FVM-Heat-Conduction/releases/tag/v1.0.0">
    <img src="https://img.shields.io/badge/Release-v1.0.0-blue.svg" alt="Release v1.0.0">
  </a>
  <img src="https://img.shields.io/badge/Rust-stable-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Method-Finite%20Volume-blue.svg" alt="Finite Volume Method">
  <a href="https://github.com/Kandil2001/Rust-FVM-Heat-Conduction/actions/workflows/ci.yml">
    <img src="https://github.com/Kandil2001/Rust-FVM-Heat-Conduction/actions/workflows/ci.yml/badge.svg" alt="Rust CI">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-lightgrey.svg" alt="MIT License">
  </a>
  <a href="https://kandil2001.github.io/projects/rust-fvm-heat-conduction.html">
    <img src="https://img.shields.io/badge/Portfolio-Case%20Study-2ea44f.svg" alt="Portfolio case study">
  </a>
</p>

A completed two-dimensional steady-state heat-conduction solver written in Rust using the cell-centered Finite Volume Method.

The project demonstrates the main numerical workflow directly: spatial discretization, boundary-condition treatment, iterative solution, convergence monitoring, structured output, and result visualization. The current stable snapshot is published as [`v1.0.0`](https://github.com/Kandil2001/Rust-FVM-Heat-Conduction/releases/tag/v1.0.0).

<p align="center">
  <img src="results/temperature.svg" alt="Finite-volume temperature field" width="700">
</p>

## Features

- cell-centered Finite Volume Method
- structured Cartesian mesh
- steady-state two-dimensional heat conduction
- fixed-temperature boundary conditions
- Gauss-Seidel iterative solution
- residual-based convergence monitoring
- CSV export for post-processing
- SVG temperature-field generation
- no external numerical or plotting libraries
- GitHub Actions build, execution, and output checks

## Problem setup

The current case models heat conduction through a square plate. The west wall is heated, while the remaining walls are maintained at a lower fixed temperature.

| Property | Value |
|---|---:|
| Domain | `0.5 m × 0.5 m` |
| Grid | `80 × 80` control volumes |
| West wall temperature | `400 K` |
| East wall temperature | `300 K` |
| North wall temperature | `300 K` |
| South wall temperature | `300 K` |
| Thermal conductivity | `1000 W/(m·K)` |
| Plate thickness | `0.01 m` |
| Convergence tolerance | `1.0e-6 K` |

The temperature difference produces a fully two-dimensional field inside the plate.

## Numerical method

For steady heat conduction with constant thermal conductivity:

```text
∇ · (k∇T) = 0
```

The equation is integrated over each control volume and written in finite-volume form:

```text
aP TP = aE TE + aW TW + aN TN + aS TS + Su
```

The prescribed wall temperatures are introduced through finite-volume source terms. The algebraic equations are solved using Gauss-Seidel iteration until the maximum temperature change between consecutive iterations falls below the configured tolerance.

## Repository structure

```text
.
├── .github/
│   └── workflows/ci.yml
├── src/
│   └── main.rs
├── results/
│   ├── temperature.csv
│   ├── residuals.csv
│   ├── summary.txt
│   └── temperature.svg
├── Cargo.toml
├── Cargo.lock
├── CITATION.cff
├── LICENSE
└── README.md
```

## Running the solver

Clone the repository and run the locked release build:

```bash
git clone https://github.com/Kandil2001/Rust-FVM-Heat-Conduction.git
cd Rust-FVM-Heat-Conduction
cargo run --release --locked
```

For GitHub Codespaces, install Rust first when `cargo` is unavailable:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo run --release --locked
```

## Automated verification

The `Rust CI` workflow runs on pushes and pull requests to `main`. It:

1. installs the stable Rust toolchain
2. checks the locked Cargo project
3. builds the release binary
4. runs the default heat-conduction case
5. verifies that the four expected result files were generated

This confirms that the tagged implementation builds and completes its default workflow. It is a software execution check, not an independent physical validation study.

## Generated output

Each successful run updates:

- `temperature.csv` — temperature at every control-volume center
- `residuals.csv` — convergence history
- `summary.txt` — simulation setup and final solver values
- `temperature.svg` — temperature-field visualization used in this README

The CSV files can be opened in Python, MATLAB, Excel, or another post-processing tool.

## Scope and limitations

This completed project focuses on a clear and compact finite-volume treatment of one two-dimensional diffusion case. It exposes coefficient assembly, boundary source terms, iterative updates, convergence checks, and output generation without hiding the numerical steps behind an external CFD library.

The current scope is intentionally limited to:

- steady conduction
- constant material properties
- a uniform structured grid
- fixed-temperature boundaries
- one Gauss-Seidel solver
- one demonstration case

Validation against an independent analytical or numerical reference was not part of the completed baseline project. The output should therefore be treated as a numerical implementation demonstration rather than a formally validated research result.

## Possible extensions

- heat-flux and convection boundary conditions
- internal heat generation
- transient conduction
- nonuniform grids
- Jacobi and Successive Over-Relaxation comparisons
- grid-refinement and independent validation studies
- parallel implementations for larger cases

## Release and citation

The first stable repository snapshot is [`v1.0.0`](https://github.com/Kandil2001/Rust-FVM-Heat-Conduction/releases/tag/v1.0.0). Use the metadata in [`CITATION.cff`](CITATION.cff) when citing the software.

## Author

Ahmed Kandil — [Portfolio](https://kandil2001.github.io/) · [LinkedIn](https://www.linkedin.com/in/ahmed-kandil03/) · [ORCID](https://orcid.org/0009-0007-2724-4565)

Released under the [MIT License](LICENSE).
