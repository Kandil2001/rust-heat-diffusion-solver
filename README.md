# 2D Heat Diffusion Solver in Rust

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Dependencies](https://img.shields.io/badge/dependencies-none-brightgreen.svg)

A small, dependency-free Rust program for solving a steady two-dimensional heat-conduction problem on a structured grid.

The model places a hot rectangular chip at the centre of a colder domain. The temperature field is then updated iteratively until the solution converges. After the run, the program saves numerical data for further analysis and creates SVG plots that can be viewed directly on GitHub.

<p align="center">
  <img src="results/temperature.svg" alt="Computed two-dimensional temperature field" width="720">
</p>

## Why I built this

This project grew from my interest in bringing numerical simulation workflows into Rust. I had previously worked on heat-transfer and CFD problems in MATLAB and wanted to rebuild a smaller problem in a language that offers strong performance, memory safety, and clear control over data structures.

The goal is not to compete with a full thermal or CFD package. Instead, the repository provides a compact and readable example of the complete scientific-computing workflow:

1. define the computational grid,
2. apply the thermal conditions,
3. solve the discretised equation iteratively,
4. monitor convergence,
5. export and visualise the results.

## Problem setup

The computational domain uses an `80 × 50` Cartesian grid.

- The outer boundaries are held at `300 K`.
- A rectangular chip in the centre is held at `360 K`.
- All remaining cells are updated until a steady temperature field is reached.

The setup is intentionally simple so that the numerical method and implementation remain easy to follow.

## Numerical method

Away from the fixed-temperature boundaries and chip region, the steady heat equation is represented by the two-dimensional Laplace equation:

```text
∂²T/∂x² + ∂²T/∂y² = 0
```

Using a uniform grid, each interior temperature is updated from its four direct neighbours:

```text
T_new(i,j) = 0.25 × [T(i+1,j) + T(i-1,j) + T(i,j+1) + T(i,j-1)]
```

The implementation uses a Jacobi-style iteration, meaning that every value in a new iteration is calculated from the previous temperature field.

Convergence is monitored using the maximum absolute temperature change:

```text
residual = max |T_new - T_old|
```

The calculation stops when this residual falls below `1.0e-6 K` or when the maximum number of iterations is reached.

More details are available in [`docs/method.md`](docs/method.md).

## Current configuration

| Parameter | Value |
|---|---:|
| Grid size | `80 × 50` |
| Cold boundary temperature | `300 K` |
| Hot chip temperature | `360 K` |
| Maximum iterations | `30,000` |
| Convergence tolerance | `1.0e-6 K` |
| Iterative method | Jacobi |

The configuration is currently defined as constants near the beginning of [`src/main.rs`](src/main.rs).

## Results

For the current setup, the solver converges in `3,701` iterations.

| Quantity | Result |
|---|---:|
| Iterations completed | `3,701` |
| Final residual | `9.988248e-7 K` |
| Minimum temperature | `300.000 K` |
| Maximum temperature | `360.000 K` |

The temperature field shows heat spreading from the central chip toward the cold outer boundaries.

### Convergence history

<p align="center">
  <img src="results/residuals.svg" alt="Residual convergence history" width="760">
</p>

The residual decreases steadily until it reaches the specified convergence tolerance.

## Getting started

### Prerequisite

Install the Rust toolchain using [rustup](https://rustup.rs/) or your operating system's package manager.

Confirm that Rust and Cargo are available:

```bash
rustc --version
cargo --version
```

### Clone and run

```bash
git clone https://github.com/Kandil2001/rust-heat-diffusion-solver.git
cd rust-heat-diffusion-solver
cargo run --release
```

The release profile is recommended because it enables compiler optimisation.

After convergence, the program prints a summary in the terminal and writes the generated files to the `results/` directory.

## Running in GitHub Codespaces

Some default Codespaces images may not have the Rust toolchain available. When `cargo` is not found, install Rust inside the Codespace and run the solver:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo run --release
```

## Running on Windows

After installing Rust with rustup, open PowerShell in the repository and run:

```powershell
cargo run --release
```

When Rust reports that a linker is missing, install the Microsoft C++ Build Tools with the **Desktop development with C++** workload, restart the terminal, and run the command again.

## Generated files

Each successful run creates or updates the following files:

| File | Description |
|---|---|
| `results/temperature.csv` | Temperature value at every grid location |
| `results/residuals.csv` | Residual recorded for every iteration |
| `results/summary.txt` | Main solver settings and final results |
| `results/temperature.svg` | Visualisation of the temperature field |
| `results/residuals.svg` | Convergence-history plot |

The CSV files can be opened in Python, MATLAB, Excel, or another post-processing tool. The SVG files require no plotting library and can be viewed directly in a browser.

## Repository structure

```text
rust-heat-diffusion-solver/
├── Cargo.toml
├── LICENSE
├── README.md
├── docs/
│   └── method.md
├── results/
│   ├── residuals.csv
│   ├── residuals.svg
│   ├── summary.txt
│   ├── temperature.csv
│   └── temperature.svg
└── src/
    └── main.rs
```

## Design choices

- **No external crates:** the solver, CSV export, and SVG generation use only the Rust standard library.
- **Flat temperature array:** the two-dimensional field is stored in a contiguous one-dimensional vector.
- **Built-in visualisation:** results can be inspected without installing Python or a plotting package.
- **Readable implementation:** the code favours clarity over advanced optimisation so that the numerical workflow remains visible.

## Scope and limitations

This is a numerical-methods demonstration rather than a complete physical heat-transfer model. The current version assumes:

- a uniform grid,
- fixed temperatures at the boundaries and chip,
- no spatial variation in material properties,
- no convection or radiation,
- no physical dimensions or dimensional heat-flux calculation,
- a purely steady-state problem.

Because the model solves a simplified Laplace problem, it should not yet be used to predict the thermal performance of a real processor or cooling system.

## Planned improvements

- Accept grid size, temperatures, and tolerance as command-line arguments.
- Add physical dimensions and thermal conductivity.
- Support heat-flux and insulated boundary conditions.
- Compare Jacobi, Gauss–Seidel, and successive over-relaxation methods.
- Add grid-independence and performance studies.
- Introduce parallel implementations for larger cases.
- Compare selected cases with an earlier MATLAB model.
- Add automated tests and continuous integration.

## License

This project is available under the [MIT License](LICENSE).

## Author

Developed by [Ahmed Kandil](https://github.com/Kandil2001) as part of an ongoing scientific-computing and numerical-simulation portfolio.
