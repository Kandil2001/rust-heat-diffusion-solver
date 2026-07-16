use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};

const NX: usize = 80;
const NY: usize = 50;
const LENGTH: f64 = 0.5;
const HEIGHT: f64 = 0.5;
const THICKNESS: f64 = 0.01;
const CONDUCTIVITY: f64 = 1000.0;

const HOT_WALL_TEMP: f64 = 360.0;
const COLD_WALL_TEMP: f64 = 300.0;
const MAX_ITERATIONS: usize = 30_000;
const TOLERANCE: f64 = 1.0e-6;

fn main() -> std::io::Result<()> {
    let dx = LENGTH / NX as f64;
    let dy = HEIGHT / NY as f64;

    let mut temperature = vec![COLD_WALL_TEMP; NX * NY];
    let mut residuals = Vec::new();

    println!("2D Finite Volume Heat Conduction Solver in Rust");
    println!("Grid: {} x {} control volumes", NX, NY);
    println!("West wall: {:.1} K", HOT_WALL_TEMP);
    println!("East wall: {:.1} K", COLD_WALL_TEMP);
    println!("North wall: {:.1} K", COLD_WALL_TEMP);
    println!("South wall: {:.1} K\n", COLD_WALL_TEMP);

    let mut iterations_done = 0;
    let mut final_residual = 0.0;

    for iteration in 1..=MAX_ITERATIONS {
        let mut max_change: f64 = 0.0;

        for j in 0..NY {
            for i in 0..NX {
                let p = index(i, j);
                let old_value = temperature[p];

                let mut a_e = CONDUCTIVITY * dy * THICKNESS / dx;
                let mut a_w = CONDUCTIVITY * dy * THICKNESS / dx;
                let mut a_n = CONDUCTIVITY * dx * THICKNESS / dy;
                let mut a_s = CONDUCTIVITY * dx * THICKNESS / dy;
                let mut source = 0.0;
                let mut source_coefficient = 0.0;

                // Fixed temperature at the heated west wall.
                if i == 0 {
                    source += 2.0 * a_w * HOT_WALL_TEMP;
                    source_coefficient -= 2.0 * a_w;
                    a_w = 0.0;
                }

                // Fixed temperature at the cold east wall.
                if i == NX - 1 {
                    source += 2.0 * a_e * COLD_WALL_TEMP;
                    source_coefficient -= 2.0 * a_e;
                    a_e = 0.0;
                }

                // Fixed temperature at the cold south wall.
                if j == 0 {
                    source += 2.0 * a_s * COLD_WALL_TEMP;
                    source_coefficient -= 2.0 * a_s;
                    a_s = 0.0;
                }

                // Fixed temperature at the cold north wall.
                if j == NY - 1 {
                    source += 2.0 * a_n * COLD_WALL_TEMP;
                    source_coefficient -= 2.0 * a_n;
                    a_n = 0.0;
                }

                let a_p = a_e + a_w + a_n + a_s - source_coefficient;
                let mut neighbour_sum = source;

                if i + 1 < NX {
                    neighbour_sum += a_e * temperature[index(i + 1, j)];
                }
                if i > 0 {
                    neighbour_sum += a_w * temperature[index(i - 1, j)];
                }
                if j + 1 < NY {
                    neighbour_sum += a_n * temperature[index(i, j + 1)];
                }
                if j > 0 {
                    neighbour_sum += a_s * temperature[index(i, j - 1)];
                }

                let new_value = neighbour_sum / a_p;
                max_change = max_change.max((new_value - old_value).abs());
                temperature[p] = new_value;
            }
        }

        residuals.push(max_change);
        iterations_done = iteration;
        final_residual = max_change;

        if iteration % 1000 == 0 {
            println!("Iteration {:>6}: residual = {:.6e} K", iteration, max_change);
        }

        if max_change < TOLERANCE {
            break;
        }
    }

    let min_temp = temperature.iter().copied().fold(f64::INFINITY, f64::min);
    let max_temp = temperature.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    println!("\nFinished after {} iterations", iterations_done);
    println!("Final residual: {:.6e} K", final_residual);
    println!("Temperature range: {:.3} to {:.3} K", min_temp, max_temp);

    create_dir_all("results")?;
    write_temperature_csv("results/temperature.csv", &temperature, dx, dy)?;
    write_residuals_csv("results/residuals.csv", &residuals)?;
    write_summary(
        "results/summary.txt",
        iterations_done,
        final_residual,
        min_temp,
        max_temp,
    )?;
    write_temperature_svg("results/temperature.svg", &temperature, min_temp, max_temp)?;

    println!("\nResults written to the results folder.");
    Ok(())
}

fn index(i: usize, j: usize) -> usize {
    j * NX + i
}

fn write_temperature_csv(
    path: &str,
    temperature: &[f64],
    dx: f64,
    dy: f64,
) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    writeln!(file, "i,j,x_m,y_m,temperature_K")?;

    for j in 0..NY {
        for i in 0..NX {
            let x = (i as f64 + 0.5) * dx;
            let y = (j as f64 + 0.5) * dy;
            writeln!(
                file,
                "{},{},{:.6},{:.6},{:.6}",
                i,
                j,
                x,
                y,
                temperature[index(i, j)]
            )?;
        }
    }
    Ok(())
}

fn write_residuals_csv(path: &str, residuals: &[f64]) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    writeln!(file, "iteration,residual_K")?;

    for (iteration, residual) in residuals.iter().enumerate() {
        writeln!(file, "{},{:.10e}", iteration + 1, residual)?;
    }
    Ok(())
}

fn write_summary(
    path: &str,
    iterations: usize,
    residual: f64,
    min_temp: f64,
    max_temp: f64,
) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    writeln!(file, "2D Finite Volume Heat Conduction Solver in Rust")?;
    writeln!(file, "Method: cell-centred finite volume method")?;
    writeln!(file, "Grid: {} x {} control volumes", NX, NY)?;
    writeln!(file, "Domain: {:.3} x {:.3} m", LENGTH, HEIGHT)?;
    writeln!(file, "West wall temperature: {:.3} K", HOT_WALL_TEMP)?;
    writeln!(file, "East wall temperature: {:.3} K", COLD_WALL_TEMP)?;
    writeln!(file, "North wall temperature: {:.3} K", COLD_WALL_TEMP)?;
    writeln!(file, "South wall temperature: {:.3} K", COLD_WALL_TEMP)?;
    writeln!(file, "Iterations: {}", iterations)?;
    writeln!(file, "Final residual: {:.6e} K", residual)?;
    writeln!(file, "Minimum temperature: {:.3} K", min_temp)?;
    writeln!(file, "Maximum temperature: {:.3} K", max_temp)?;
    Ok(())
}

fn write_temperature_svg(
    path: &str,
    temperature: &[f64],
    min_temp: f64,
    max_temp: f64,
) -> std::io::Result<()> {
    let cell = 10;
    let width = NX * cell;
    let height = NY * cell;
    let mut file = BufWriter::new(File::create(path)?);

    writeln!(
        file,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    )?;
    writeln!(file, "<title>Finite-volume temperature field</title>")?;

    for j in 0..NY {
        for i in 0..NX {
            let value = temperature[index(i, j)];
            let ratio = if max_temp > min_temp {
                (value - min_temp) / (max_temp - min_temp)
            } else {
                0.0
            };
            let red = (255.0 * ratio) as u8;
            let blue = (255.0 * (1.0 - ratio)) as u8;
            let x = i * cell;
            let y = (NY - 1 - j) * cell;

            writeln!(
                file,
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgb({},0,{})"/>"#,
                x, y, cell, cell, red, blue
            )?;
        }
    }

    writeln!(file, "</svg>")?;
    Ok(())
}
