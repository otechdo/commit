#![allow(clippy::multiple_crate_versions)]

use cargo_metadata::MetadataCommand;
use chrono::Utc;
use inquire::{Confirm, MultiSelect, Select, Text};
use std::env::consts::OS;
use std::fs::{self, read_to_string, remove_file, File};
use std::io::Write;
use std::path::Path;
use std::path::MAIN_SEPARATOR_STR;
use std::process::Command;
use std::vec;
const DEV_BRANCH: &str = "develop";
const FEATURE_BRANCH_PREFIX: &str = "feature";

const LANG: &str = "en_US";
const HELP: [&str; 68] = [
    "Star: New feature or enhancement",
    "Comet: Bug fix or error resolution",
    "Nebula: Code refactoring",
    "Pulsar: Performance improvement",
    "Quasar: Documentation or clarity improvement",
    "Asteroid Belt: Code cleanup and maintenance",
    "Solar Flare: Testing-related changes",
    "Dwarf Planet: Minor updates or fixes",
    "Terraform: Infrastructure changes",
    "Black Hole: Removing large chunks of code or features",
    "Wormhole: Merging branches or connecting code parts",
    "Big Bang: Initial commit or major feature start",
    "Launch: Deploying to production or releasing a version",
    "Lightspeed: Significant performance improvements",
    "Mission Control: Project management changes",
    "Spacewalk: Urgent hotfixes",
    "Moon Landing: Major milestone or goal completion",
    "First Contact: Initial integrations with external systems",
    "Interstellar Communication: Improving documentation or communication",
    "Solar Eclipse: Temporarily masking functionality",
    "Supernova: Major, transformative change",
    "Meteor Shower: Series of small changes or fixes",
    "Solar Wind: Refactoring code structure",
    "Lunar Eclipse: Temporarily disabling a feature",
    "Cosmic Dawn: Initial implementation of a feature",
    "Solar Storm: Rapid, impactful changes",
    "Lunar Transit: Minor, temporary change",
    "Perihelion: Brings the project closer to its goals or objectives",
    "Aphelion: Immediate goals, but is necessary for long-term progress",
    "White Dwarf: Improving code comments or documentation",
    "Red Giant: Expanding a feature or functionality",
    "Neutron Star: Optimizing code for performance",
    "Binary Star: Merging features or components",
    "Brown Dwarf: Undeveloped feature with potential",
    "Quark Star: Experimental or speculative change",
    "Rogue Planet: Independent change",
    "Stellar Nursery: Creation of new components",
    "Planetary Nebula: Removal or deprecation of a component",
    "Globular Cluster: Collection of related changes",
    "Void: Removal of a module, component, or feature",
    "Gravity: Resolving merge conflicts or dependencies",
    "Dark Matter: Fixing unknown or mysterious bugs",
    "Time Dilation: Improving code performance",
    "Spacetime: Changes to date, time, or scheduling",
    "Gravitational Lensing: Altering data or information flow",
    "Cosmic String: Connecting code parts",
    "Quantum Fluctuation: Small, random change",
    "Hawking Radiation: Removing technical debt",
    "Quantum Entanglement: Establishing close relationships between code parts",
    "Gravitational Redshift: Slowing down or reducing code performance",
    "Space Probe: Testing new features or technologies",
    "Station: Creating or improving environments",
    "Rocket Launch: Deploying to production",
    "Spacewalk: Urgent production hotfixes",
    "Space Elevator: Making codebase more accessible",
    "Warp Drive: Significant speed improvement",
    "Dyson Sphere: Comprehensive optimization of a specific area",
    "Generation Ship: Long-term project for a self -sustaining system",
    "Lagrange Point: Stabilizing or balancing code parts",
    "Orbital Maneuver: Changing project direction",
    "Mission Control: Represents project management-related changes",
    "Moon Landing: Celebrates the completion of major milestones",
    "Interstellar Travel: Migration to a new architecture or language",
    "Rover: Exploration of new technologies or approaches",
    "Singularity: Resolution of a complex or hard-to-reproduce issue",
    "Relativity: Changes related to time, dates, or timestamps",
    "Expansion: Scaling up the system or increasing capacity",
    "Big Crunch: Reduction of codebase size or removal of features",
];

fn check(sentence: &str) -> bool {
    let mut f = File::create("/tmp/commiter").expect("msg");
    writeln!(f, "{sentence}").expect("msg");
    let o = Command::new("hunspell")
        .arg("-d")
        .arg(LANG)
        .arg("-l")
        .arg("/tmp/commiter")
        .output()
        .expect("msg")
        .stdout;
    o.is_empty()
}
fn get_last_tag() -> String {
    let tag: String = String::from_utf8(
        Command::new("git")
            .arg("describe")
            .arg("--tags")
            .arg("--abbrev=0")
            .current_dir(".")
            .output()
            .unwrap()
            .stdout,
    )
    .expect("Faile to find a TAG");
    let data: Vec<&str> = tag.split('\n').collect();
    (*data.first().expect("msg")).to_string()
}
fn get_log() -> String {
    let log = File::create("log").expect("failed to create log");
    let d = format!("{}..HEAD", get_last_tag());
    assert!(Command::new("git")
        .arg("log")
        .arg("--format=fuller")
        .arg(d.as_str())
        .stdout(log)
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    read_to_string("log").expect("failed to parse file")
}

fn program_or_lib() -> String {
    if read_to_string("Cargo.toml")
        .expect("no cargo project")
        .contains("lib")
    {
        String::from("library")
    } else {
        String::from("software")
    }
}
fn create_changelog() -> bool {
    if Path::new("./logs").is_dir().eq(&false) {
        fs::create_dir_all("./logs").expect("msg");
    }
    let filename = format!(
        "./logs{MAIN_SEPARATOR_STR}{}-{}-changes.md",
        project(),
        version()
    );
    let logs = get_log();
    let lines = logs.lines();
    let mut f = File::create(filename.as_str()).expect("failed to create file");
    writeln!(
        f,
        "# 🚀 {} {} released\n\nToday the `{}`, we are very happy to present the **{}** version of our `{}` {} !\n\nThis release marks a significant step forward for our {} {}.\n\n## Demonstration\n\n{}\n\n## What it's?\n\nIt's {}\n\n## What we do ?\n\n- {}\n\n## Our team\n\n- {}\n\n## Links\n\n- [Source code]({})\n- [Home]({})\n- [Issues]({})\n- [Pull Request]({})\n- [Discussions]({})\n- [Wiki]({})\n- [Projects]({})\n- [Releases]({})\n- [Crates.io](https://crates.io/crates/{}/{})\n",
        project(),
        version(),
        Utc::now().date_naive(),
        version(),
        project(),
        program_or_lib(),
        program_or_lib(),
        project(),
        project(),
        description(),
        keywords().join("\n- "),
        authors().join("\n- "),
        repository(),
        homepage(),
        issues(),
        pulls_request(),
        discussions(),
        wiki(),
        projects(),
        releases(),
        project(),
        version()
    )
    .expect("msg");
    for t in commit_types_with_help() {
        let ttt: Vec<&str> = t.split(':').collect();
        let title: String = (*ttt.last().unwrap()).to_string();
        writeln!(f, "###{title}\n").expect("msg");
        for line in lines.clone() {
            let current = (*ttt.first().unwrap()).to_string();
            if line.contains(current.as_str()) {
                let lll = line.split('\n');
                for l in lll {
                    let c = l.replace(ttt.first().unwrap(), "");
                    let cc: Vec<&str> = c.split(':').collect();
                    let ccc: Vec<&str> = cc.last().unwrap().split('\n').collect();
                    let message = ccc.join("\n");
                    writeln!(f, "\n  -{message}").expect("msg");
                }
            }
        }
    }
    writeln!(
        f,
        "\n## README\n\n{}\n\n## LICENSE\n\n```\n{}\n```",
        read_to_string(readme())
            .expect("readme file not founded")
            .trim(),
        read_to_string(license())
            .expect("LICENSE file not founded")
            .trim()
    )
    .expect("msg");
    remove_file("log").expect("failed to remove log");
    Path::new(filename.as_str()).exists()
}
fn issues() -> String {
    let mut x = repository();
    if x.contains("github") {
        x.push_str("/issues");
    } else if x.contains("gitlab") {
        x.push_str("-/issues");
    }
    x
}

fn wiki() -> String {
    let mut x = repository();
    if x.contains("github") {
        x.push_str("/wiki");
    } else if x.contains("gitlab") {
        x.push_str("-/wikis");
    }
    x
}
fn projects() -> String {
    let mut x = repository();
    if x.contains("github") {
        x.push_str("/projects");
    }
    x
}

fn pulls_request() -> String {
    let mut x = repository();
    if x.contains("github") {
        x.push_str("/pulls");
    } else if x.contains("gitlab") {
        x.push_str("-/merge_requests");
    }
    x
}

fn discussions() -> String {
    let mut x = repository();
    if x.contains("github") {
        x.push_str("/discussions");
    }
    x
}

fn create_patch() {
    if Path::new("./patches").exists().eq(&false) {
        assert!(fs::create_dir_all("./patches").is_ok());
    }
    assert!(Command::new("git")
        .arg("format-patch")
        .arg("-1")
        .current_dir("./patches")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}
fn create_tag() {
    let m: String = Text::new("Enter the tag message : ").prompt().unwrap();

    let v: String = Text::new("Enter the tag version : ")
        .with_default(version().as_str())
        .prompt()
        .unwrap();

    if m.is_empty() || v.is_empty() {
        create_tag();
    }
    assert!(Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(v.as_str())
        .arg("-m")
        .arg(m.as_str())
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    assert!(Command::new("cargo")
        .arg("publish")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}

fn send_tag() {
    assert!(Command::new("git")
        .arg("push")
        .arg("--all")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    assert!(Command::new("git")
        .arg("push")
        .arg("--tags")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}
fn commit(m: &str) -> bool {
    assert!(Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(m)
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    create_patch();
    if confirm("Create new tag", false) {
        create_tag();
        send_tag();
    }
    true
}

fn diff() -> bool {
    Command::new("git")
        .arg("diff")
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn fmt() {
    assert!(Command::new("cargo")
        .arg("fmt")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    clear();
}

fn zuu() -> bool {
    clear();

    if Path::new("Cargo.toml").exists() {
        fmt();
        if Command::new("zuu")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success()
        {
            clear();
            return true;
        }
        return false;
    }
    clear();
    true
}

fn version() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package = metadata.packages.first().unwrap();
    package.version.to_string()
}

fn dependencies() -> Vec<String> {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    let dependency_names: Vec<String> = package
        .dependencies
        .iter()
        .map(|dep| dep.name.clone())
        .collect();
    dependency_names
}

fn releases() -> String {
    let mut x = repository();
    if x.contains("github") {
        x.push_str("/releases");
    } else if x.contains("gitlab") {
        x.push_str("-/tags");
    }
    x
}
fn project() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package.name.to_string()
}

fn keywords() -> Vec<String> {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package.keywords.clone()
}

fn homepage() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package.clone().homepage.expect("no homepage")
}

fn readme() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package
        .clone()
        .readme
        .expect("no readme define")
        .to_string()
}

fn license() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package
        .clone()
        .license_file
        .expect("no licences define")
        .to_string()
}

fn repository() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package.clone().repository.expect("no repository define")
}

///
/// # Panics
///
fn description() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package
        .description
        .as_ref()
        .expect("missing description")
        .to_string()
}

fn authors() -> Vec<String> {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package.authors.clone()
}
fn clear() {
    if OS.eq("windows") {
        assert!(Command::new("cls")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    } else {
        assert!(Command::new("clear")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

fn commit_types_with_help() -> [&'static str; 68] {
    let mut x = HELP;
    x.sort_unstable();
    x
}

fn commit_scope() -> String {
    let mut scope: String;
    loop {
        scope = Text::new("Please enter the commit scope : ")
            .prompt()
            .unwrap();
        if scope.is_empty() {
            continue;
        }
        if scope.len().gt(&20) {
            println!("scope can be superior to 20 character");
            continue;
        }
        if confirm(
            format!("Really use the commit scope : {scope}").as_str(),
            false,
        ) {
            break;
        }
    }
    scope
}

fn commit_types() -> String {
    let mut t: String;
    loop {
        t = Select::new(
            "Please enter the commit type : ",
            commit_types_with_help().to_vec(),
        )
        .prompt()
        .unwrap()
        .to_string();
        if t.is_empty() {
            continue;
        }
        if confirm(format!("Really use the commit type : {t}").as_str(), false) {
            break;
        }
    }
    let x: Vec<&str> = t.split(':').collect();
    (*x.first().unwrap()).to_string()
}

fn commit_summary() -> String {
    let mut summary: String;
    loop {
        summary = Text::new("Please enter the commit summary : ")
            .prompt()
            .unwrap();
        if summary.is_empty() {
            continue;
        }
        if summary.len().gt(&50) {
            println!("Summary must be contains less than 50 chararacter");
            continue;
        }
        if confirm(format!("Use the summary : {summary}").as_str(), false) {
            break;
        }
    }
    summary
}

fn commit_description() -> String {
    let mut description: String = String::new();
    loop {
        let d = Text::new("Please enter the commit description : ")
            .prompt()
            .unwrap();
        if d.is_empty() {
            continue;
        }
        description.push_str(format!("{d}\n\n").as_str());
        if confirm("Add a new description line", false) {
            continue;
        }
        break;
    }
    description
}

fn commit_why() -> String {
    let mut why: String = String::new();
    loop {
        let w = Text::new("Please explain the reasoning behind the change : ")
            .prompt()
            .unwrap();
        if w.is_empty() {
            continue;
        }
        if w.len().gt(&50) {
            println!("The reasoning behind the change must be contains less than 50 chararacter");
            continue;
        }
        why.push_str(format!("\n\t* {w}").as_str());
        if confirm("Continue to write the changes : ", false) {
            continue;
        }
        break;
    }
    why
}
fn commit_footer() -> String {
    let mut footer: String = String::new();
    if confirm("Code has breaking changes ?", false) {
        footer.push_str("BREAKING CHANGE: ");
        loop {
            let b = Text::new("Please enter the breaking change description: ")
                .prompt()
                .unwrap();
            if b.is_empty() {
                continue;
            }
            if confirm(
                format!("Use breaking change description : {b}").as_str(),
                false,
            ) {
                footer.push_str(b.as_str());
                break;
            }
        }
    }
    if confirm("Code has resolving issues ?", false) {
        loop {
            footer.push_str("\n\tFixes ");
            let f = Text::new("Please enter the issue number : ")
                .prompt()
                .unwrap();
            if f.is_empty() {
                continue;
            }
            footer.push_str(format!("#{f}\n").as_str());
            if confirm("Code resolving an other issues ?", false) {
                continue;
            }
            break;
        }
    }
    if confirm("Code resolve an issue ?", false) {
        loop {
            footer.push_str("\n\tCloses ");
            let f = Text::new("Please enter the issue number : ")
                .prompt()
                .unwrap();
            if f.is_empty() {
                continue;
            }
            footer.push_str(format!("#{f}\n").as_str());
            if confirm("Code resolve an other issue ?", false) {
                continue;
            }
            break;
        }
    }
    footer
}

fn bad_sentences() {
    println!("The entered text is not correct: it must be written in English and not contain any errors.");
}
fn get_scope() -> String {
    let mut scope: String;
    loop {
        scope = commit_scope();
        if check(scope.as_str()) {
            break;
        }
        bad_sentences();
    }
    scope
}

fn get_summary() -> String {
    let mut summary: String;
    loop {
        summary = commit_summary();
        if check(summary.as_str()) {
            break;
        }
        bad_sentences();
    }
    summary
}

fn get_description() -> String {
    let mut description: String;
    loop {
        description = commit_description();
        if check(description.as_str()) {
            break;
        }
        bad_sentences();
    }
    description
}

fn get_why() -> String {
    let mut why: String;
    loop {
        why = commit_why();
        if check(why.as_str()) {
            break;
        }
        bad_sentences();
    }
    why
}
fn get_footer() -> String {
    let mut footer: String;
    loop {
        footer = commit_footer();
        if check(footer.as_str()) {
            break;
        }
        bad_sentences();
    }
    footer
}
fn prepare_commit() -> bool {
    let c = format!(
        "{}({}): {}\n\n{}\n\nThe following changes were made:\n\t{}\n\nThe changes :\n{}\n\nCo-authored-by: {} <{}>",
        commit_types(),
        get_scope(),
        get_summary(),
        get_description(),
        get_why(),
        get_footer(),
        name(),
        email()
    );
    commit(c.as_str())
}

fn confirm(msg: &str, default: bool) -> bool {
    Confirm::new(msg)
        .with_default(default)
        .prompt()
        .unwrap()
        .eq(&true)
}

fn email() -> String {
    String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.email")
            .current_dir(".")
            .output()
            .expect("git email not found")
            .stdout,
    )
    .expect("msg")
    .trim()
    .to_string()
}

fn name() -> String {
    String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.name")
            .current_dir(".")
            .output()
            .expect("username not found")
            .stdout,
    )
    .expect("msg")
    .trim()
    .to_string()
}

enum Verb {
    Start,
    Finish,
}

fn checkout(b: &str) -> bool {
    Command::new("git")
        .arg("checkout")
        .arg(b)
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}
fn create_branch(b: &str) -> bool {
    Command::new("git")
        .arg("branch")
        .arg(b)
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn remove_branch(b: &str) -> bool {
    Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(b)
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn add() -> bool {
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn init() -> bool {
    create_branch(DEV_BRANCH) && checkout(DEV_BRANCH)
}

fn merge(branch: &str) -> bool {
    Command::new("git")
        .arg("merge")
        .arg(branch)
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}
fn start_feature(name: &str) -> bool {
    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(format!("{FEATURE_BRANCH_PREFIX}/{name}").as_str())
        .arg(DEV_BRANCH)
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}
fn finish_feature(name: &str) -> bool {
    assert!(checkout(DEV_BRANCH));
    assert!(pull(DEV_BRANCH));
    assert!(merge(format!("{FEATURE_BRANCH_PREFIX}/{name}").as_str()));
    assert!(remove_branch(
        format!("{FEATURE_BRANCH_PREFIX}/{name}").as_str()
    ));
    true
}

fn ask(msg: &str) -> String {
    let mut x: String;
    loop {
        x = Text::new(msg).prompt().unwrap().to_string();
        if x.is_empty() {
            continue;
        }
        break;
    }
    x
}

fn feature(name: &str, v: &Verb) -> bool {
    match v {
        Verb::Start => start_feature(name),
        Verb::Finish => finish_feature(name),
    }
}

fn pull(branch: &str) -> bool {
    Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(branch)
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}
fn stash() -> bool {
    Command::new("git")
        .arg("stash")
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn install_program() -> bool {
    Command::new("cargo")
        .arg("install")
        .arg("--path")
        .arg(".")
        .current_dir(".")
        .spawn()
        .expect("cargo")
        .wait()
        .unwrap()
        .success()
}

fn run_program() -> bool {
    Command::new("cargo")
        .arg("run")
        .current_dir(".")
        .spawn()
        .expect("cargo")
        .wait()
        .unwrap()
        .success()
}

fn update() -> bool {
    Command::new("cargo")
        .arg("update")
        .current_dir(".")
        .spawn()
        .expect("cargo")
        .wait()
        .unwrap()
        .success()
}

fn remove_dependencies() -> bool {
    let dependencies = MultiSelect::new("Select dependencies to remove : ", dependencies())
        .prompt()
        .unwrap();
    if dependencies.is_empty() {
        return remove_dependencies();
    }
    for d in &dependencies {
        assert!(Command::new("cargo")
            .arg("rm")
            .arg(d.as_str())
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .unwrap()
            .success());
    }
    true
}

fn test_application() -> bool {
    Command::new("cargo")
        .arg("test")
        .arg("-j")
        .arg("4")
        .arg("--")
        .arg("--show-output")
        .current_dir(".")
        .spawn()
        .expect("cargo")
        .wait()
        .unwrap()
        .success()
}

fn publish() -> bool {
    Command::new("cargo")
        .arg("publish")
        .current_dir(".")
        .spawn()
        .expect("cargo")
        .wait()
        .unwrap()
        .success()
}

fn send() -> bool {
    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("--all")
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
        && Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("--tags")
            .current_dir(".")
            .spawn()
            .expect("git")
            .wait()
            .unwrap()
            .success()
}
fn status() -> bool {
    Command::new("git")
        .arg("status")
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn flow(zuu: bool) {
    loop {
        clear();
        if zuu {
            let o: &str = Select::new(
                "What you want do :  ",
                vec![
                    "Init a repository",
                    "Start a new feature",
                    "Finish a feature",
                    "Commit",
                    "Generate change log",
                    "Send modifications",
                    "Show status",
                    "Show branches",
                    "Show diff",
                    "Show logs",
                    "Run tests",
                    "Run program",
                    "Remove dependencies",
                    "Publish",
                    "Install",
                    "Update dependencies",
                    "Stash all modifications",
                    "Delete a branch",
                    "Delete a tag",
                    "Create a new branch with no staged modifications",
                    "Show tags",
                    "Add modifications",
                    "Quit",
                ],
            )
            .prompt()
            .unwrap();
            match o {
                "Init a repository" => assert!(init()),

                "Start a new feature" => assert!(feature(
                    ask("Enter the feature name").as_str(),
                    &Verb::Start
                )),
                "Finish a feature" => assert!(feature(
                    ask("Enter the feature name").as_str(),
                    &Verb::Finish
                )),
                "Commit" => assert!(prepare_commit()),
                "Generate change log" => assert!(create_changelog()),
                "Send modifications" => assert!(send()),
                "Show status" => assert!(status()),
                "Show branches" => assert!(display_branches()),
                "Show diff" => assert!(diff()),
                "Show logs" => assert!(logs()),
                "Run tests" => assert!(test_application()),
                "Run program" => assert!(run_program()),
                "Remove dependencies" => assert!(remove_dependencies()),
                "Publish" => assert!(publish()),
                "Install" => assert!(install_program()),
                "Stash all modifications" => assert!(stash()),
                "Delete a branch" => assert!(remove_branch(
                    ask("Enter the name of the branch to remove : ").as_str()
                )),
                "Delete a tag" => assert!(delete_tag()),
                "Update dependencies" => assert!(update()),
                "Create a new branch with no staged modifications" => assert!(stash_branch()),
                "Show tags" => assert!(tags()),
                "Add modifications" => assert!(add()),
                "Quit" => break,
                _ => {
                    continue;
                }
            }
        }
    }
    println!("Bye...");
}

fn stash_branch() -> bool {
    assert!(stash());
    let feat = ask("Enter the feature name");
    Command::new("git")
        .arg("stash")
        .arg("branch")
        .arg(feat.as_str())
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn tags() -> bool {
    Command::new("git")
        .arg("tag")
        .arg("--list")
        .arg("--sort=-taggerdate")
        .arg("--format=%(refname:short) | %(objectname) | %(taggerdate:short) | %(subject)")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn delete_tag() -> bool {
    let tag = ask("Enter the name of the tag to delete : ");
    Command::new("git")
        .arg("tag")
        .arg("-d")
        .arg(tag.as_str())
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn display_branches() -> bool {
    Command::new("git")
        .arg("show-branch")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn logs() -> bool {
    Command::new("git")
        .arg("log")
        .current_dir(".")
        .spawn()
        .expect("git")
        .wait()
        .unwrap()
        .success()
}

fn main() {
    flow(zuu());
}
