mod exercises;
mod solvers;

use exercises::newton::cooling_law::{
    analytical::CoolingLaw,
    differential::CoolingDifferential,
    parameters::CoolingParams,
};

use solvers::euler;

use std::fs::File;
use std::io::Write;

struct Exercises;

impl Exercises {

    pub fn test_newton() -> std::io::Result<()> {

        const TIME: f64 = 5.0;
        const STEP: f64 = 0.01;

        let mut file = File::create("iterations.txt")?;
        let mut params = CoolingParams {
                env_temperature: 20.0,
                initial_temperature: 90.0,
                k: None,
            };

        for i in 0..=800 {

            let k = i as f64 * STEP;

            params.set_k(k);

            let model = CoolingLaw::new(params.clone())
                .with_temperature_at(TIME);

            let output = format!("{}", model);

            println!("{}", output);

            writeln!(file, "{}", output)?;
        }

        Ok(())
    }

    pub fn test_euler_newton() {

        let mut temp = 90.0;

        let mut params = CoolingParams {
            env_temperature: 20.0,
            initial_temperature: temp,
            k: None,
        };

        params.set_k(0.1);

        let equation = CoolingDifferential::new(params);

        let solver = euler::Euler::new(equation, 0.1);

        let mut t = 0.0;

        for _ in 0..800 {

            temp = solver.step(t, temp);
            t += 0.1;

            println!("t={:.2} T={:.2}", t, temp);
        }
    }

    pub fn compare_analytical_vs_euler() {

        const STEP: f64 = 0.1;
        const ITERATIONS: usize = 100;

        let mut temp_euler = 90.0;
        let mut t = 0.0;

        let mut params = CoolingParams {
            env_temperature: 20.0,
            initial_temperature: 90.0,
            k: None,
        };
        
        params.set_k(0.1);

        let differential = CoolingDifferential::new(params.clone());
        let solver = euler::Euler::new(differential, STEP);

        let mut analytical_model = CoolingLaw::new(params);

        println!(
            "{:<8} {:<15} {:<15} {:<15}",
            "t", "Analytical", "Euler", "Error"
        );

        for _ in 0..ITERATIONS {

            let analytical = analytical_model
                .temperature_at(t)
                .unwrap();

            let euler_temp = temp_euler;

            let error = (analytical - euler_temp).abs();

            println!(
                "{:<8.2} {:<15.6} {:<15.6} {:<15.6}",
                t,
                analytical,
                euler_temp,
                error
            );

            temp_euler = solver.step(t, temp_euler);
            t += STEP;
        }
}
}

fn main() {

    Exercises::test_newton().unwrap();

    Exercises::test_euler_newton();

    Exercises::compare_analytical_vs_euler();

}