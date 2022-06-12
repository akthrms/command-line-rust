use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

const PROG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const THE_BUSTLE: &str = "tests/inputs/the_bustle.txt";

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn usage() -> TestResult {
    for option in &["-h", "--help"] {
        Command::cargo_bin(PROG)?
            .arg(option)
            .assert()
            .success()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PROG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PROG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty_b() -> TestResult {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_b() -> TestResult {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> TestResult {
    run(&["-n", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders_b() -> TestResult {
    run(&["-b", SPIDERS], "tests/expected/spiders.txt.b.out")
}

#[test]
fn the_bustle() -> TestResult {
    run(&[THE_BUSTLE], "tests/expected/the_bustle.txt.out")
}

#[test]
fn the_bustle_n() -> TestResult {
    run(&["-n", THE_BUSTLE], "tests/expected/the_bustle.txt.n.out")
}

#[test]
fn the_bustle_b() -> TestResult {
    run(&["-b", THE_BUSTLE], "tests/expected/the_bustle.txt.b.out")
}

#[test]
fn all() -> TestResult {
    run(&[EMPTY, FOX, SPIDERS, THE_BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> TestResult {
    run(
        &["-n", EMPTY, FOX, SPIDERS, THE_BUSTLE],
        "tests/expected/all.n.out",
    )
}

#[test]
fn all_b() -> TestResult {
    run(
        &["-b", EMPTY, FOX, SPIDERS, THE_BUSTLE],
        "tests/expected/all.b.out",
    )
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PROG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn the_bustle_stdin() -> TestResult {
    run_stdin(
        THE_BUSTLE,
        &["-"],
        "tests/expected/the_bustle.txt.stdin.out",
    )
}

#[test]
fn the_bustle_stdin_n() -> TestResult {
    run_stdin(
        THE_BUSTLE,
        &["-n", "-"],
        "tests/expected/the_bustle.txt.n.stdin.out",
    )
}

#[test]
fn the_bustle_stdin_b() -> TestResult {
    run_stdin(
        THE_BUSTLE,
        &["-b", "-"],
        "tests/expected/the_bustle.txt.b.stdin.out",
    )
}
