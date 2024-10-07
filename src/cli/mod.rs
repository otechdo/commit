use clap::{Arg, ArgMatches, Command};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use glob::glob;
use inquire::{Editor, Text};
use regex::Regex;
use reqwest::blocking::get;
use std::fs::{File, remove_file, read_to_string};
use std::collections::HashMap;
use std::io::{Write, copy, Read, BufReader};
use std::path::Path;
use std::process::Command as V;
use tar::{Archive, Builder};
use tera::Tera;
use std::io::Result;
use chrono::Local;

fn parse_templates(dir: &str) -> Result<HashMap<String, Vec<String>>> {
    let mut templates: HashMap<String, Vec<String>> = HashMap::new();
    let pattern = format!("{dir}/**/*.txt");
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                let mut file = File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let re = Regex::new(r"\{\{(.*?)\}\}").unwrap();
                let mut variables: Vec<String> = Vec::new();

                for caps in re.captures_iter(&content) {
                    variables.push(caps[1].trim().to_string());
                }
                templates.insert(file_name, variables);
            }
            Err(e) => println!("Error reading file: {e:?}"),
        }
    }
    Ok(templates)
}

#[cfg(feature = "mercurial")]

fn get_hg_parent_commit() -> Option<String> {
    let output = V::new("hg")
        .arg("log")
        .arg("-r")
        .arg("p1(.)")
        .arg("--template")
        .arg("{node}")
        .output()
        .expect("Failed to execute hg command");

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}
fn get_git_parent_commit() -> Option<String> {
    let output = V::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=%H")
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

#[doc = "The main of cli"]
pub async fn run_cli() {
    let matches = cli_app().get_matches();
    match matches.subcommand() {
        Some(("commit", sub_matches)) => commit(sub_matches).await,
        Some(("init", sub_matches)) => init(sub_matches).await,
        Some(("archive", sub_matches)) => create_archive(sub_matches).await,
        Some(("template", sub_matches)) => manage_templates(sub_matches).await,
        _ => println!("Unknown command. Use 'commit', 'init', 'archive', or template."),
    }
}
#[doc = "the command arguments"]
fn cli_app() -> Command {
    Command::new("commiter")
        .about("A CLI to commit changes in repositories and manage templates")
        .subcommand(
            Command::new("archive")
                .about("Create a tar.gz archive from a directory")
                .arg(clap::Arg::new("directory")
                    .short('d')
                    .long("directory")
                    .help("Path to the directory to archive")
                    .required(true)
                )
                .arg(clap::Arg::new("output")
                    .short('o')
                    .long("output")
                    .help("Path for the output tar.gz file")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("commit")
                .about("Commit changes in the repository")
                .arg(clap::Arg::new("template")
                    .short('t')
                    .long("template")
                    .help("Template name to use for the commit message (default, bugfix, feature)")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("init")
                .about("Initialize templates by downloading and extracting an archive")
        ).subcommand(
        Command::new("template")
            .about("Manage templates")
            .subcommand(Command::new("list").about("List available templates"))
            .subcommand(Command::new("add").about("Add a new template"))
            .subcommand(
                Command::new("remove")
                    .about("Remove a template")
                    .arg(Arg::new("template").short('t').long("template").required(true).help("the template to remove"))
            )
            .subcommand(
                Command::new("edit")
                    .about("Edit an existing template")
                    .arg(Arg::new("template").short('t').long("template").required(true).help("the template to edit"))
            )
    )
}
#[doc = "create an tar.gz archive from a directory"]
async fn create_archive(matches: &ArgMatches) {
    if let Some(directory) = matches.get_one::<String>("directory") {
        if let Some(output) = matches.get_one::<String>("output") {
            println!("Creating archive from directory: {}", directory);
            println!("Output archive path: {}", output);
            match tar_directory(directory, output) {
                Ok(_) => println!("Archive created successfully."),
                Err(e) => eprintln!("Failed to create archive: {}", e),
            }
        }
    }
}

#[doc = "create an tar.gz archive from a directory"]
fn tar_directory<P: AsRef<Path>>(directory: P, output: P) -> Result<()> {
    let tar_gz = File::create(output)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_dir_all(".", directory)?;

    Ok(())
}

async fn commit(matches: &ArgMatches) {
    #[cfg(target_os = "linux")]
    assert!(V::new("clear").spawn().is_ok());
    #[cfg(not(target_os = "linux"))]
    assert!(V::new("cls").spawn().is_ok());
    if let Some(template) = matches.get_one::<String>("template") {
        let templates = parse_templates("templates").expect("Failed to parse templates");
        let tera = Tera::new("templates/**/*").expect("Failed to load templates");
        let mut context = tera::Context::new();

        if let Some(variables) = templates.get(template.as_str()) {
            for variable in variables {
                if variable.ne("parent_commit") && variable.ne("commit_date") {
                    loop {
                        let user_input = Text::new(&format!("Enter value for {variable}:"))
                            .prompt()
                            .unwrap();
                        if !user_input.is_empty() {
                            context.insert(variable, &user_input);
                            break;
                        }
                        println!("{variable} must be defined");
                    }
                } else {
                    let current_date = Local::now().format("%Y-%m-%d").to_string();
                    context.insert("commit_date", &current_date);

                    #[cfg(feature = "mercurial")]
                    {
                        if let Some(parent_commit) = get_hg_parent_commit() {
                            context.insert(variable, &parent_commit);
                        } else {
                            context.insert(variable, "No parent commit found");
                        }
                    }
                    #[cfg(feature = "fossil")]
                    {
                        if let Some(parent_commit) = get_fossil_parent_commit() {
                            context.insert(variable, &parent_commit);
                        } else {
                            context.insert(variable, "No parent commit found");
                        }
                    }
                    #[cfg(feature = "pijul")]
                    {
                        if let Some(parent_commit) = get_pijul_parent_commit() {
                            context.insert(variable, &parent_commit);
                        } else {
                            context.insert(variable, "No parent commit found");
                        }
                    }
                    #[cfg(feature = "git")]
                    {
                        if let Some(parent_commit) = get_git_parent_commit() {
                            context.insert(variable, &parent_commit);
                        } else {
                            context.insert(variable, "No parent commit found");
                        }
                    }
                }
            }

            // Rendre le template
            let commit_message = tera.render(&format!("{template}.txt"), &context)
                .expect("Failed to render template");

            // Commit selon le VCS activé
            #[cfg(feature = "git")]
            commit_vcs("git", &commit_message);
            #[cfg(feature = "mercurial")]
            commit_vcs("hg", &commit_message);
            #[cfg(feature = "fossil")]
            commit_vcs("fossil", &commit_message);
            #[cfg(feature = "pijul")]
            commit_vcs("pijul", &commit_message);
        } else {
            eprintln!("Template not found: {template}");
        }
    } else {
        eprintln!("No template specified.");
    }
}
#[cfg(feature = "fossil")]
fn get_fossil_parent_commit() -> Option<String> {
    let output = V::new("fossil")
        .arg("info")
        .arg("current")
        .output()
        .expect("Failed to execute fossil command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(parent_line) = output_str.lines().find(|line| line.starts_with("parent:")) {
            let parent_commit = parent_line.split_whitespace().nth(1);
            return parent_commit.map(|p| p.to_string());
        }
    }
    None
}


#[cfg(feature = "pijul")]
fn get_pijul_parent_commit() -> Option<String> {
    let output = V::new("pijul")
        .arg("log")
        .arg("-n")
        .arg("1")
        .output()
        .expect("Failed to execute pijul command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        output_str.lines().nth(0).map(|s| s.to_string())
    } else {
        None
    }
}

async fn manage_templates(arg_matches: &ArgMatches) {
    if let Some(_) = arg_matches.subcommand_matches("list") {
        let templates = parse_templates("templates").expect("Failed to parse templates");
        for (template, vars) in &templates {
            println!("Template : {}", template);
            if vars.is_empty() {
                println!("  No variables found for this template.");
            } else {
                println!("  Variables :");
                for var in vars {
                    println!("    - {}", var);
                }
            }
            println!();
        }
    } else if let Some(_) = arg_matches.subcommand_matches("add") {
        let template_name = Text::new("Enter the new template name:").prompt().unwrap();
        let template_content = Editor::new("Enter the template content:").prompt().unwrap();

        let template_path = format!("templates/{template_name}.txt");
        let mut file = File::create(template_path).expect("Failed to create template file");
        writeln!(file, "{}", template_content).expect("Failed to write to template file");
        println!("Template '{template_name}' has been added.");
    } else if let Some(remove_matches) = arg_matches.subcommand_matches("remove") {
        if let Some(template) = remove_matches.get_one::<String>("template") {
            let template_path = format!("templates/{template}.txt");
            if Path::new(&template_path).exists() {
                remove_file(&template_path).expect("Failed to remove template");
                println!("Template '{template}' has been removed.");
            } else {
                println!("Template '{template}' does not exist.");
            }
        }
    } else if let Some(edit_matches) = arg_matches.subcommand_matches("edit") {
        if let Some(template) = edit_matches.get_one::<String>("template") {
            let template_path = format!("templates/{template}.txt");

            if Path::new(&template_path).exists() {
                let current_content =
                    read_to_string(&template_path).expect("Failed to read template");
                let new_content = Editor::new("Enter the new content for the template:")
                    .with_editor_command("vim".as_ref())
                    .with_predefined_text(current_content.as_str())
                    .prompt()
                    .unwrap();
                let mut file =
                    File::create(&template_path).expect("Failed to open template for editing");
                writeln!(file, "{}", new_content).expect("Failed to write new content to template");
                println!("Template '{template}' has been updated.");
            } else {
                println!("Template '{template}' does not exist.");
            }
        }
    } else {
        println!("No template operation specified.");
    }
}

#[doc = "save commit message"]
fn commit_vcs(vcs: &str, message: &str) {
    if let Ok(mut commit) = V::new(vcs)
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(".")
        .spawn()
    {
        if let Ok(comm) = commit.wait() {
            if comm.success() {
                println!("Committed successfully");
            } else {
                eprintln!("Failed to commit message");
            }
        }
    } else {
        eprintln!("Error running {} command.", vcs);
    }
}

#[doc = "download and extract archive"]
async fn init(_: &ArgMatches) {
    let url = "https://raw.githubusercontent.com/otechdo/commit/refs/heads/main/templates.tar.gz"; // Remplace par l'URL réelle
    let output_archive = "templates.tar.gz";
    let output_dir = "templates";
    download_file(url, output_archive).expect("Failed to download archive");
    extract_archive(output_archive, output_dir).expect("Failed to extract archive");
    remove_file(output_archive).expect("Failed to remove archive");
    println!("Templates successfully initialized.");
}

#[doc = "download archive"]
fn download_file(url: &str, output_path: &str) -> Result<()> {
    let response = get(url).expect("Failed to send request");
    let mut file = File::create(output_path)?;
    copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;
    Ok(())
}

#[doc = "extract archive"]
fn extract_archive(archive_path: &str, output_dir: &str) -> Result<()> {
    let file = File::open(archive_path)?;
    let decompressor = GzDecoder::new(BufReader::new(file));
    let mut archive = Archive::new(decompressor);
    archive.unpack(output_dir)?;
    Ok(())
}
