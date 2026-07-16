# 2D Finite Volume Heat Conduction Solver in Rust

A two-dimensional steady-state heat conduction solver written in Rust using the cell-centred Finite Volume Method (FVM). The project covers the main numerical workflow from spatial discretisation and boundary-condition treatment to iterative solution, convergence monitoring, and result visualisation.

<p align="center">
  <img src="results/temperature.svg" alt="Finite-volume temperature field" width="700">
</p>

## Features

- Cell-centred Finite Volume Method
- Structured Cartesian mesh
- Steady-state two-dimensional heat conduction
- Fixed-temperature boundary conditions
- Gauss–Seidel iterative solution
- Residual-based convergence monitoring
- CSV export for post-processing
- SVG temperature-field generation
- No external numerical or plotting libraries

## Problem setup

The current case models heat conduction through a square plate. The west wall is heated, while the remaining three walls are maintained at a lower fixed temperature.

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

The difference between the heated wall and the three colder walls produces a fully two-dimensional temperature field inside the plate.

## Numerical method

For steady heat conduction with constant thermal conductivity, the governing equation is

```text
∇ · (k∇T) = 0
```

The equation is integrated over each control volume and written in the standard finite-volume form

```text
aP TP = aE TE + aW TW + aN TN + aS TS + Su
```

The prescribed wall temperatures are introduced through finite-volume source terms. The resulting algebraic equations are solved using Gauss–Seidel iteration until the maximum temperature change between two consecutive iterations falls below the specified tolerance.

## Repository structure

```text
.
├── src/
│   └── main.rs
├── results/
│   ├── temperature.csv
│   ├── residuals.csv
│   ├── summary.txt
│   └── temperature.svg
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

## Running the solver

Clone the repository and run the release build:

```bash
git clone https://github.com/Kandil2001/rust-fvm-heat-conduction.git
cd rust-fvm-heat-conduction
cargo run --release
```

For GitHub Codespaces, install Rust first when `cargo` is not available:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo run --release
```

## Generated output

Each successful run updates the files inside the `results/` directory:

- `temperature.csv` — temperature at every control-volume centre
- `residuals.csv` — convergence history
- `summary.txt` — simulation setup and final solver values
- `temperature.svg` — temperature-field visualisation displayed in this README

The CSV files can be opened in Python, MATLAB, Excel, or another post-processing tool.

## Project scope

The current implementation focuses on a clear and compact finite-volume treatment of a two-dimensional diffusion problem. It provides a direct view of the coefficient assembly, boundary source terms, iterative update, convergence check, and output generation without hiding the numerical steps behind an external CFD library.

The solver is intentionally limited to one steady conduction case, but its structure can be extended to support more advanced heat-transfer and numerical studies.

## Future development

- Heat-flux and convection boundary conditions
- Internal heat generation
- Transient heat conduction
- Non-uniform grids
- Jacobi and Successive Over-Relaxation comparisons
- Grid-independence and performance studies
- Validation against an analytical or MATLAB reference solution
- Parallel implementations for larger cases

## License

This project is available under the [MIT License](LICENSE).

## Author

[Ahmed Kandil](https://github.com/Kandil2001)
