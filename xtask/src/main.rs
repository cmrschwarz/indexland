// Helper command for testing indexland with different sets of feature
// flags enabled. Running `cargo xtask` without any options
// will run every test that's also runnning in ci.

use std::{fmt::Write, process::Command};

use clap::Parser;

#[derive(Parser)]
#[command(about = "run different test configurations for indexland")]
struct Cli {
    #[arg(short, long, default_value = "false")]
    test: bool,

    #[arg(short, long, default_value = "false")]
    format: bool,

    #[arg(short, long, default_value = "false")]
    docs_rs: bool,

    #[arg(short, long, default_value = "false")]
    clippy: bool,
}

struct FeatureSet {
    name: &'static str,
    features: &'static [&'static str],
}

static FEATURE_SETS: &[FeatureSet] = &[
    FeatureSet {
        name: "no std, no alloc",
        features: &[],
    },
    FeatureSet {
        name: "no std, no alloc, nonmax",
        features: &["nonmax"],
    },
    FeatureSet {
        name: "no std, no alloc, arrayvec",
        features: &["arrayvec"],
    },
    FeatureSet {
        name: "no std, alloc",
        features: &["alloc"],
    },
    // smallvec requires std so it's not in here
    FeatureSet {
        name: "no std, alloc, indexmap, arrayvec",
        features: &["alloc", "indexmap", "arrayvec"],
    },
    FeatureSet {
        name: "std",
        features: &["std"],
    },
    FeatureSet {
        name: "std, nonmax",
        features: &["std", "nonmax"],
    },
    FeatureSet {
        name: "std, indexmap, arrayvec, smallvec",
        features: &["alloc", "indexmap", "arrayvec", "smallvec"],
    },
];

fn run_cargo_with_env<'a>(
    env: impl IntoIterator<Item = (&'a str, &'a str)>,
    args: impl IntoIterator<Item = &'a str>,
) {
    let args = args.into_iter().collect::<Vec<_>>();

    println!("⚙️  running `cargo {}`", args.join(" "));

    let status = Command::new("cargo")
        .args(&args)
        .env_remove("CARGO") // otherwise +nightly will be propagated correctly in docs-rs
        .envs(env)
        .status()
        .expect("Failed to execute cargo");

    if !status.success() {
        println!("❌ Command Failed: `cargo {}`", args.join(" "));
        std::process::exit(1);
    }
}

fn run_cargo<'a>(args: impl IntoIterator<Item = &'a str>) {
    run_cargo_with_env([("RUSTFLAGS", "-D warnings")], args);
}

fn run_cargo_nightly<'a>(args: impl IntoIterator<Item = &'a str>) {
    run_cargo_with_env(
        [
            ("RUSTFLAGS", "-D warnings"),
            ("RUSTUP_TOOLCHAIN", "nightly"),
        ],
        // add +nightly aswell mainly so it shows up in the log
        std::iter::once("+nightly").chain(args),
    );
}

fn run_cargo_with_features<'a>(setup: impl IntoIterator<Item = &'a str>, features: &str) {
    let mut args = setup.into_iter().collect::<Vec<_>>();

    args.push("--no-default-features");
    args.push("--features");
    args.push(features);

    run_cargo(args);
}

fn run_tests() {
    for feature_set in FEATURE_SETS {
        println!(
            "\n⚡ Testing indexland feature set: {} ⚡",
            feature_set.name
        );

        run_cargo_with_features(
            ["test", "--tests", "-p=indexland"],
            &feature_set.features.join(","),
        );

        println!(
            "\n⚡ Testing indexland_derive feature set: {} ⚡",
            feature_set.name
        );

        run_cargo_with_features(
            ["test", "--tests", "-p=indexland_derive"],
            &feature_set
                .features
                .iter()
                .fold(String::new(), |mut res, f| {
                    res.write_fmt(format_args!("indexland/{f},")).unwrap();
                    res
                }),
        );
    }

    println!("\n⚡ Testing doc-tests ⚡");
    run_cargo_with_features(["test", "--doc", "--workspace"], "indexland/full");

    println!("\n⚡ Testing with full features ⚡");
    run_cargo_with_features(["test", "--workspace"], "indexland/full");

    println!("\n⚡ Testing with full features and --release ⚡");
    run_cargo_with_features(["test", "--workspace", "--release"], "indexland/full");

    let examples = std::fs::read_dir("./examples")
        .expect("Failed to read examples directory")
        .filter_map(Result::ok)
        .map(|entry| {
            entry
                .file_name()
                .to_str()
                .unwrap()
                .split(".rs")
                .next()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>();

    for ex in &examples {
        println!("\n⚡ Testing example {ex} ⚡");
        run_cargo_with_features(["run", "--example", ex], "full");
    }
}

fn main() {
    let mut args = Cli::parse();

    if !args.clippy && !args.format && !args.docs_rs && !args.test {
        args.clippy = true;
        args.format = true;
        args.docs_rs = true;
        args.test = true;
    }

    if args.clippy {
        println!("\n🧐 check + clippy 📎");
        run_cargo_nightly(["check", "--workspace"]);
        run_cargo_nightly(["clippy", "--workspace"]);
    }

    if args.format {
        println!("\n🧹 rust-fmt 🧹");
        run_cargo_nightly(["fmt", "--check", "--all"]);
    }

    if args.docs_rs {
        println!("\n📝 docs-rs for indexland 📝");
        run_cargo_nightly(["docs-rs", "-p=indexland"]);

        println!("\n📝 docs-rs for indexland 📝");
        run_cargo_nightly(["docs-rs", "-p=indexland_derive"]);
    }

    if args.test {
        run_tests();
    }

    println!("✅ All actions successful");
}
