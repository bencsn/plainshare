use colored::Colorize;

pub fn new(project_name: String) {
    println!(
        "{}",
        format!("🔨 Creating a new project at: {}", project_name).purple().italic()
    );
    // create a new project directory
    let is_project_name_valid = is_project_name_valid(&project_name);
    if !is_project_name_valid.0 {
        println!("{}", format!("{}",is_project_name_valid.1).red());
        return;
    }
    // Prompt user for project description
    let project_description = prompt_for_project_description();
    create_new_project_directory(&project_name);

    create_routes_directory(&project_name);

    // create a toml file called _config.toml
    create_config_file(&project_name, &project_description);

    println!("{}",format!("🚀🚀 Project created successfully! 🚀🚀").green().bold());
    println!();
    println!("{}", format!("👉 To start the server, run: ").bold().italic());
    println!("Run `cd {}` to enter the project directory", project_name);
    println!("Run `plainshare build .` to build the project");
    println!("Run `plainshare start .` to start the development server");
}

fn is_project_name_valid(project_name: &String) -> (bool, String) {
    // if project directory already exists, return error
    let project_dir = std::path::Path::new(&project_name);
    if project_dir.exists() {
        return (
            false,
            "🙅 Project directory already exists. Aborting.".to_string(),
        );
    }
    return (true, "".to_string());
}

fn prompt_for_project_description() -> String {
    let mut project_description = String::new();
    println!("📝 Enter a description for your project {}:", format!("(default: A PlainShare project)").italic().yellow());
    std::io::stdin()
        .read_line(&mut project_description)
        .expect("Failed to read line");
    // escape quotes including single quotes
    project_description = project_description.replace("\"", "\\\"");
    // escape new lines
    project_description = project_description.trim_end().to_string();

    if project_description.is_empty() {
        project_description = "A PlainShare project".to_string();
    }
    return project_description;
}

fn create_new_project_directory(project_name: &String) {
    let project_dir = std::path::Path::new(&project_name);
    std::fs::create_dir(project_dir).expect("🙅‍♂️ Failed to create project directory.");
}

fn create_config_file(project_name: &String, project_description: &String) {
    let config_file_path = std::path::Path::new(&project_name).join("_config.toml");
    let mut config_file =
        std::fs::File::create(config_file_path).expect("🙅‍♂️ Failed to create config file.");
    let config_file_contents = format!(
        r#"
name = "{}"
description = "{}"
build_target = "build"
"#,
        project_name, project_description
    );
    std::io::Write::write_all(&mut config_file, config_file_contents.as_bytes())
        .expect("🙅‍♂️ Failed to write to config file.");
}

// TODO: create files from here instead
fn create_routes_directory(project_name: &String) {
    let project_dir = std::path::Path::new(&project_name);
    // create a routes directory
    let routes_dir = project_dir.join("routes");
    std::fs::create_dir(&routes_dir).expect("🙅‍♂️ Failed to create routes directory.");

    // add index.md to routes directory
    let index_file_path = routes_dir.join("index.md");
    let mut index_file =
        std::fs::File::create(index_file_path).expect("🙅‍♂️ Failed to create index file.");

    let index_file_contents = r#"
---
title: "Hello World"
---
# Hello World
This is a home page.

[Next page](/second-page)
    "#;
    std::io::Write::write_all(&mut index_file, index_file_contents.as_bytes())
        .expect("🙅‍♂️ Failed to write to index file.");

    // add second-page.md to routes directory
    let second_page_file_path = routes_dir.join("second-page.md");
    let mut second_page_file = std::fs::File::create(second_page_file_path)
        .expect("🙅‍♂️ Failed to create second page file.");

    let second_page_file_contents = r#"
---
title: "Second Page"
---
# Second Page
This is a second page.

[Home page](/)
    "#;
    std::io::Write::write_all(&mut second_page_file, second_page_file_contents.as_bytes())
        .expect("🙅‍♂️ Failed to write to second page file.");
}
