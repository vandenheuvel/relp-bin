use std::convert::TryInto;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

use clap::Parser;
use relp::algorithm::{OptimizationResult, SolveRelaxation};
use relp::algorithm::two_phase::matrix_provider::MatrixProvider;
use relp::algorithm::two_phase::tableau::inverse_maintenance::carry::Carry;
use relp::algorithm::two_phase::tableau::inverse_maintenance::carry::lower_upper::LUDecomposition;
use relp::data::linear_program::elements::LinearProgramType;
use relp::data::linear_program::general_form::GeneralForm;
use relp::data::linear_program::general_form::Scalable;
use relp::io::error::Import;
use relp::io::mps;
use relp::io::mps::MPS;
use relp_num::{Rational64, RationalBig};

/// An exact linear program solver written in rust.
#[derive(Parser)]
#[clap(about, version)]
struct Args {
    /// File containing the problem description
    problem_file: String,
    /// Disable presolving
    #[clap(long)]
    no_presolve: bool,
    /// Disable scaling
    #[clap(long)]
    no_scale: bool,
    /// Parsing mode that requires MPS fields to be in an exact column (2, 5, 15, 25, 40 and 50)
    #[clap(long)]
    fixed_parse_mode: bool,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.problem_file);
    println!("Reading problem file: \"{}\"...", path.to_string_lossy());

    // Open and read the file
    let program = read_to_string(path)
        .map_err(Import::IO)
        .expect("Couldn't read the file");

    let mps: MPS<Rational64> = if args.fixed_parse_mode {
        mps::parse_fixed(&program)
    } else {
        mps::parse_free(&program)
    }
        .expect("Couldn't parse the file");

    let mut general: GeneralForm<RationalBig> = mps.try_into()
        .expect("Problem is inconsistent");

    if !args.no_presolve {
        println!("Presolving...");
        if let Err(program_type) = general.presolve() {
            match program_type {
                LinearProgramType::FiniteOptimum(solution) => {
                    println!("Solution computed during presolve.\n{}", solution.to_string())
                },
                LinearProgramType::Infeasible => println!("Problem is not feasible."),
                LinearProgramType::Unbounded => println!("Problem is unbounded."),
            }
            exit(0);
        }
    }

    let constraint_type_counts = general.standardize();

    let scaling = if !args.no_scale {
        println!("Scaling...");
        Some(general.scale())
    } else {
        None
    };

    let data = general.derive_matrix_data(constraint_type_counts);

    println!("Solving relaxation...");
    let result = data.solve_relaxation::<Carry<RationalBig, LUDecomposition<_>>>();

    println!("Solution computed:");
    match result {
        OptimizationResult::FiniteOptimum(vector) => {
            let mut reconstructed = data.reconstruct_solution(vector);
            if let Some(scaling) = scaling {
                scaling.scale_back(&mut reconstructed);
                general.scale_back(scaling);
            }
            let solution = general.compute_full_solution_with_reduced_solution(reconstructed);
            println!("{}", solution.to_string());
        },
        OptimizationResult::Infeasible => println!("Problem is not feasible."),
        OptimizationResult::Unbounded => println!("Problem is unbounded."),
    }
}
