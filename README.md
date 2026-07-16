# 2D Finite Volume Heat Conduction Solver in Rust

A simple steady-state heat conduction solver written in Rust using the cell-centred finite volume method.

The model represents a square plate heated from the west side. The west wall is fixed at `400 K`, while the east, north, and south walls are fixed at `300 K`. This gives a clear two-dimensional temperature field inside the plate.

<p align="center">
  <img src="results/temperature.svg" alt="Finite-volume temperature field" width="760">
</p>

## Problem setup

- Domain: `0.5 m × 0.5 m`
- Grid: `80 × 80` control volumes
- West wall temperature: `400 K`
- East wall temperature: `300 K`
- North wall temperature: `300 K`
- South wall temperature: `300 K`
- Thermal conductivity: `1000 W/(m K)`
- Plate thickness: `0.01 m`

This is one simple case based on the finite-volume heat-transfer models I worked with in MATLAB during my bachelor thesis.

## Numerical method

The steady heat conduction equation is integrated over each control volume:

```text
∇ · (k∇T) = 0
```

The discretised equation is written in the standard finite-volume form:

```text
aP TP = aE TE + aW TW + aN TN + aS TS + Su
```

The fixed wall temperatures are applied through finite-volume source terms. The equations are solved using a Gauss–Seidel iteration.

The solver stops when the maximum temperature change between two iterations is below `1.0e-6 K`.

## Run the solver

Install Rust, clone the repository, and run:

```bash
git clone https://github.com/Kandil2001/rust-heat-diffusion-solver.git
cd rust-heat-diffusion-solver
cargo run --release
```

For GitHub Codespaces, install Rust first when `cargo` is not available:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo run --release
```

## Output files

The solver writes the results to the `results/` folder:

- `temperature.csv` – temperature at each control-volume centre
- `residuals.csv` – convergence history
- `summary.txt` – case settings and final results
- `temperature.svg` – temperature-field visualisation

The program uses only the Rust standard library.

## Why I made this

I wanted to rebuild one of the simple finite-volume heat-transfer cases from my bachelor thesis in Rust. The aim was to keep the code clear and show the main numerical steps without using a large CFD library.

This is not a complete thermal model of a processor or cooling system. It is a small project for practising finite-volume discretisation and scientific programming in Rust.

## Possible next steps

- Add convection boundary conditions
- Add heat-flux boundary conditions
- Compare Gauss–Seidel with Jacobi and SOR
- Add grid-independence and performance studies
- Compare the Rust results with the earlier MATLAB model

## License

This project is available under the [MIT License](LICENSE).

## Author

[Ahmed Kandil](https://github.com/Kandil2001)
