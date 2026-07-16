# 2D Heat Conduction Solver in Rust

A simple steady-state heat conduction solver written in Rust using the cell-centred finite volume method.

The case represents a rectangular plate heated from the west side. The east side is kept cold, while the north and south sides are insulated. Heat therefore moves mainly from the hot wall toward the cold wall.

## Problem setup

- Domain size: `0.5 m × 0.5 m`
- Grid: `80 × 50` control volumes
- West wall: `360 K`
- East wall: `300 K`
- North wall: insulated
- South wall: insulated
- Thermal conductivity: `1000 W/(m K)`

This is a small educational case based on the finite-volume diffusion models I developed in MATLAB during my bachelor thesis.

## Numerical method

The steady heat conduction equation is integrated over every control volume:

```text
∇ · (k∇T) = 0
```

The discretised equation is written as:

```text
aP TP = aE TE + aW TW + aN TN + aS TS + Su
```

The wall temperatures are included through the finite-volume source terms. The insulated top and bottom walls have zero normal heat flux.

The equations are solved iteratively using a Gauss–Seidel update. The calculation stops when the maximum temperature change is below `1.0e-6 K`.

## Run the solver

Install Rust, clone the repository and run:

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

This is not a complete thermal model of a processor or cooling system. It is a small numerical-method project for practising finite-volume discretisation and scientific programming in Rust.

## License

This project is available under the [MIT License](LICENSE).

## Author

[Ahmed Kandil](https://github.com/Kandil2001)
