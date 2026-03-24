mod exercises;
mod solvers;
mod utils;

use crate::solvers::data::model::Model;
use crate::solvers::least_squares::builder::LogisticLeastSquaresBuilder;
use crate::solvers::least_squares::solver::LeastSquaresSolver;
use crate::exercises::logistic_model::learned::LearnedModel;

use exercises::newton::cooling_law::{
    analytical::CoolingLaw,
    differential::CoolingDifferential,
    parameters::CoolingParams,
};

use exercises::logistic_model::{ analytical::LogisticAnalytical, parameters::LogisticParams };
use solvers::data::DataGenerator;

use solvers::euler;

use utils::linspace::Linspace;

use std::fs::File;
use std::io::Write;

use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::{Circle, PathElement};
use plotters::series::LineSeries;
use plotters::style::{BLUE, RED, WHITE};
use plotters::style::Color;

use crate::exercises::logistic_model::differential::LogisticDifferential;


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

        let temp = 90.0;

        let mut params = CoolingParams {
            env_temperature: 20.0,
            initial_temperature: temp,
            k: None,
        };

        params.set_k(0.1);

        let equation = CoolingDifferential::new(params);

        let solver = euler::Euler::new(&equation, 0.1);

        let env_temp = equation.get_params().env_temperature;

        for state in solver.iterate(0.0, (&equation).get_params().initial_temperature).take_while(|state| (state.y - env_temp).abs() > 0.0001) {
            println!("t={:.10} T={:.10}", state.t, state.y);
        }
    }

    pub fn test_euler_logistic_model(){

        let y0 = 0.1;

        let mut params = LogisticParams::new(y0);

        let equation = LogisticDifferential::new(params);
        
        let solver = euler::Euler::new(&equation, 0.2);

        let points: Vec<(f64, f64)> = solver
            .iterate(0.0, y0)
            .take_while(|state| (10.0 - state.t).abs() >= 0.02)
            .map(|state| (state.t, state.y))
            .collect();

        let root = BitMapBackend::new("euler.png", (800, 600)).into_drawing_area();
            root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Euler Logistic", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..10.0, 0.0..1.2)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(
        points.iter().map(|&(t, y)| Circle::new((t, y), 3, BLUE.filled()))
    ).unwrap();

    let analytical = LogisticAnalytical::new(LogisticParams::new(y0));

    chart.draw_series(LineSeries::new(
        points.iter().map(|&(t, _)| (t, analytical.evaluate(t))),
        &RED,
    )).unwrap();


    }

    pub fn compare_analytical_vs_euler() {

        const STEP: f64 = 0.1;
        const ITERATIONS: usize = 100;
        let t = 0.0;
        let t0 = t;

        let mut params = CoolingParams {
            env_temperature: 20.0,
            initial_temperature: 90.0,
            k: None,
        };
        
        params.set_k(0.1);

        let differential = CoolingDifferential::new(params.clone());
        let solver = euler::Euler::new(&differential, STEP);

        let mut analytical_model = CoolingLaw::new(params);

        println!(
            "{:<8} {:<15} {:<15} {:<15}",
            "t", "Analytical", "Euler", "Error"
        );

        for state_euler in solver.iterate(t0, differential.get_params().initial_temperature).take(ITERATIONS) {

            let analytical = analytical_model
                .temperature_at(state_euler.t)
                .unwrap();

            let euler_temp = state_euler.y;

            let error = (analytical - euler_temp).abs();

            println!(
                "{:<8.2} {:<15.6} {:<15.6} {:<15.6}",
                state_euler.t,
                analytical,
                euler_temp,
                error
            );
        };
    }

    pub fn generate_experimental_data_logistic_model() {

        let n = 50;
        let t_min = 0.0;
        let t_max = 10.0;
        let linspace = Linspace::new(t_min, t_max, n);

        let t_values: Vec<f64> = linspace.generate();

        let params = LogisticParams { y0: 0.1 };
        let model = LogisticAnalytical::new(params);

        let generator = DataGenerator::new(model);
        let data = generator.generate(&t_values, 0.1); 

        let y_clean: Vec<f64> = t_values
            .iter()
            .map(|&t| 1.0 / (1.0 + 9.0 * (-t).exp()))
            .collect();

        let root = BitMapBackend::new("logistic.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Logistic Model", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(t_min..t_max, 0.0..1.2)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart.draw_series(LineSeries::new(
            t_values.iter().zip(y_clean.iter()).map(|(&t, &y)| (t, y)),
            &RED,
        )).unwrap();

        chart.draw_series(
            data.iter().map(|&(t, y)| Circle::new((t, y), 3, BLUE.filled()))
        ).unwrap();

    }

    pub fn test_learned_model() {
        let n = 50;
        let t_min = 0.0;
        let t_max = 10.0;
        let h = 0.2;
        let y0 = 0.1;
        let linspace = Linspace::new(t_min, t_max, n);
        let t_values: Vec<f64> = linspace.generate();
        
        let params = LogisticParams { y0: y0 };
        let model = LogisticAnalytical::new(params);

        let generator = DataGenerator::new(model);
        let data = generator.generate(&t_values, 0.1); 

        let  lq = LogisticLeastSquaresBuilder::build(&data, h);
       
        let coeffs = LeastSquaresSolver::solve(&lq.A, &lq.r);

        let a0 = coeffs[0];
        let a1 = coeffs[1];
        let a2 = coeffs[2];

        println!("a0={}, a1={}, a2={}", a0, a1, a2);

        let learned = LearnedModel { a0, a1, a2 };

        let solver = euler::Euler::new(&learned, h);

        let points: Vec<(f64, f64)> = solver
            .iterate(0.0, y0)
            .take_while(|state| (10.0 - state.t).abs() >= 0.02)
            .map(|state| (state.t, state.y))
            .collect();

        let root = BitMapBackend::new("euler.png", (800, 600)).into_drawing_area();
            root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Euler Logistic", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0.0..10.0, 0.0..1.2)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart.draw_series(
            points.iter().map(|&(t, y)| Circle::new((t, y), 3, BLUE.filled()))
        ).unwrap();

        let analytical = LogisticAnalytical::new(LogisticParams::new(y0));

        chart.draw_series(LineSeries::new(
            points.iter().map(|&(t, _)| (t, analytical.evaluate(t))),
            &RED,
        )).unwrap();
    }
}

fn main() {

    // Exercises::test_newton().unwrap();

    // Exercises::test_euler_newton();

    // Exercises::compare_analytical_vs_euler();
    /*
    Exercises::generate_experimental_data_logistic_model();

    Exercises::test_euler_logistic_model();
     */
    Exercises::test_learned_model();

}