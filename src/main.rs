use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};

const NX: usize = 80;
const NY: usize = 50;
const COLD_TEMP: f64 = 300.0;
const CHIP_TEMP: f64 = 360.0;
const MAX_ITERATIONS: usize = 30_000;
const TOLERANCE: f64 = 1.0e-6;

fn main() -> std::io::Result<()> {
    println!("Rust Heat Diffusion Solver");
    println!("--------------------------");
    println!("Grid: {} x {}", NX, NY);
    println!("Cold boundary temperature: {:.1} K", COLD_TEMP);
    println!("Hot chip temperature: {:.1} K", CHIP_TEMP);
    println!();

    let mut temperature = vec![COLD_TEMP; NX * NY];
    let mut residuals = Vec::new();
    apply_hot_chip(&mut temperature);

    let mut final_residual = 0.0;
    let mut iterations_done = 0;

    for iteration in 1..=MAX_ITERATIONS {
        let old_temperature = temperature.clone();
        let mut max_change: f64 = 0.0;

        for j in 1..NY - 1 {
            for i in 1..NX - 1 {
                if is_inside_hot_chip(i, j) {
                    continue;
                }

                let new_value = 0.25
                    * (old_temperature[index(i + 1, j)]
                        + old_temperature[index(i - 1, j)]
                        + old_temperature[index(i, j + 1)]
                        + old_temperature[index(i, j - 1)]);

                let change = (new_value - old_temperature[index(i, j)]).abs();
                max_change = max_change.max(change);
                temperature[index(i, j)] = new_value;
            }
        }

        final_residual = max_change;
        iterations_done = iteration;
        residuals.push(max_change);

        if iteration % 1000 == 0 {
            println!("Iteration {:>6}: residual = {:.6e} K", iteration, max_change);
        }

        if max_change < TOLERANCE {
            break;
        }
    }

    let min_temp = temperature.iter().copied().fold(f64::INFINITY, f64::min);
    let max_temp = temperature.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    println!();
    println!("Finished.");
    println!("Iterations: {}", iterations_done);
    println!("Final residual: {:.6e} K", final_residual);
    println!("Minimum temperature: {:.3} K", min_temp);
    println!("Maximum temperature: {:.3} K", max_temp);

    create_dir_all("results")?;
    write_temperature_csv("results/temperature.csv", &temperature)?;
    write_residuals_csv("results/residuals.csv", &residuals)?;
    write_summary(
        "results/summary.txt",
        iterations_done,
        final_residual,
        min_temp,
        max_temp,
    )?;
    write_temperature_svg("results/temperature.svg", &temperature, min_temp, max_temp)?;
    write_residuals_svg("results/residuals.svg", &residuals)?;

    println!();
    println!("Results saved in the results folder.");

    Ok(())
}

fn index(i: usize, j: usize) -> usize {
    j * NX + i
}

fn apply_hot_chip(temperature: &mut [f64]) {
    for j in 0..NY {
        for i in 0..NX {
            if is_inside_hot_chip(i, j) {
                temperature[index(i, j)] = CHIP_TEMP;
            }
        }
    }
}

fn is_inside_hot_chip(i: usize, j: usize) -> bool {
    let chip_width = NX / 5;
    let chip_height = NY / 5;

    let i_min = NX / 2 - chip_width / 2;
    let i_max = NX / 2 + chip_width / 2;
    let j_min = NY / 2 - chip_height / 2;
    let j_max = NY / 2 + chip_height / 2;

    i >= i_min && i <= i_max && j >= j_min && j <= j_max
}

fn write_temperature_csv(path: &str, temperature: &[f64]) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "i,j,temperature_K")?;

    for j in 0..NY {
        for i in 0..NX {
            writeln!(writer, "{},{},{:.6}", i, j, temperature[index(i, j)])?;
        }
    }

    Ok(())
}

fn write_residuals_csv(path: &str, residuals: &[f64]) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "iteration,residual_K")?;

    for (iteration, residual) in residuals.iter().enumerate() {
        writeln!(writer, "{},{:.10e}", iteration + 1, residual)?;
    }

    Ok(())
}

fn write_summary(
    path: &str,
    iterations: usize,
    final_residual: f64,
    min_temp: f64,
    max_temp: f64,
) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);

    writeln!(writer, "Rust Heat Diffusion Solver")?;
    writeln!(writer, "==========================")?;
    writeln!(writer, "Grid: {} x {}", NX, NY)?;
    writeln!(writer, "Cold boundary temperature: {:.3} K", COLD_TEMP)?;
    writeln!(writer, "Hot chip temperature: {:.3} K", CHIP_TEMP)?;
    writeln!(writer, "Maximum iterations: {}", MAX_ITERATIONS)?;
    writeln!(writer, "Tolerance: {:.3e} K", TOLERANCE)?;
    writeln!(writer, "Iterations completed: {}", iterations)?;
    writeln!(writer, "Final residual: {:.6e} K", final_residual)?;
    writeln!(writer, "Minimum temperature: {:.3} K", min_temp)?;
    writeln!(writer, "Maximum temperature: {:.3} K", max_temp)?;

    Ok(())
}

fn write_temperature_svg(
    path: &str,
    temperature: &[f64],
    min_temp: f64,
    max_temp: f64,
) -> std::io::Result<()> {
    let cell_size = 10usize;
    let width = NX * cell_size;
    let height = NY * cell_size;
    let mut writer = BufWriter::new(File::create(path)?);

    writeln!(
        writer,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    )?;
    writeln!(writer, r#"<title>Temperature field</title>"#)?;

    for j in 0..NY {
        for i in 0..NX {
            let temperature_value = temperature[index(i, j)];
            let ratio = if max_temp > min_temp {
                (temperature_value - min_temp) / (max_temp - min_temp)
            } else {
                0.0
            };

            let (red, green, blue) = temperature_colour(ratio);
            let x = i * cell_size;
            let y = (NY - 1 - j) * cell_size;

            writeln!(
                writer,
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgb({},{},{})"/>"#,
                x, y, cell_size, cell_size, red, green, blue
            )?;
        }
    }

    writeln!(writer, "</svg>")?;
    Ok(())
}

fn temperature_colour(ratio: f64) -> (u8, u8, u8) {
    let ratio = ratio.clamp(0.0, 1.0);
    let red = (255.0 * ratio) as u8;
    let green = (180.0 * (1.0 - (2.0 * ratio - 1.0).abs())) as u8;
    let blue = (255.0 * (1.0 - ratio)) as u8;

    (red, green, blue)
}

fn write_residuals_svg(path: &str, residuals: &[f64]) -> std::io::Result<()> {
    let width = 900.0;
    let height = 500.0;
    let margin = 60.0;
    let plot_width = width - 2.0 * margin;
    let plot_height = height - 2.0 * margin;
    let mut writer = BufWriter::new(File::create(path)?);

    writeln!(
        writer,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{:.0}" height="{:.0}" viewBox="0 0 {:.0} {:.0}">"#,
        width, height, width, height
    )?;
    writeln!(writer, r#"<title>Residual history</title>"#)?;
    writeln!(writer, r#"<rect width="100%" height="100%" fill="white"/>"#)?;
    writeln!(
        writer,
        r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="none" stroke="black" stroke-width="2"/>"#,
        margin, margin, plot_width, plot_height
    )?;

    if residuals.len() > 1 {
        let min_log = residuals
            .iter()
            .copied()
            .filter(|value| *value > 0.0)
            .map(f64::log10)
            .fold(f64::INFINITY, f64::min);

        let max_log = residuals
            .iter()
            .copied()
            .filter(|value| *value > 0.0)
            .map(f64::log10)
            .fold(f64::NEG_INFINITY, f64::max);

        write!(writer, r#"<polyline fill="none" stroke="black" stroke-width="2" points=""#)?;

        for (iteration, residual) in residuals.iter().enumerate() {
            let x_ratio = iteration as f64 / (residuals.len() - 1) as f64;
            let residual_log = residual.max(1.0e-20).log10();
            let y_ratio = if max_log > min_log {
                (residual_log - min_log) / (max_log - min_log)
            } else {
                0.0
            };

            let x = margin + x_ratio * plot_width;
            let y = margin + (1.0 - y_ratio) * plot_height;
            write!(writer, "{:.2},{:.2} ", x, y)?;
        }

        writeln!(writer, r#""/>"#)?;
    }

    writeln!(
        writer,
        r#"<text x="{:.1}" y="{:.1}" font-size="22" font-family="Arial">Residual history</text>"#,
        margin, 35.0
    )?;
    writeln!(
        writer,
        r#"<text x="{:.1}" y="{:.1}" font-size="16" font-family="Arial">Iteration</text>"#,
        width / 2.0 - 35.0,
        height - 15.0
    )?;
    writeln!(
        writer,
        r#"<text x="15" y="{:.1}" font-size="16" font-family="Arial" transform="rotate(-90 15,{:.1})">log10 residual</text>"#,
        height / 2.0 + 45.0,
        height / 2.0 + 45.0
    )?;

    writeln!(writer, "</svg>")?;
    Ok(())
}