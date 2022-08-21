use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;

pub fn new(project_name: String) {
    println!("Creating new project: {}", project_name);
    // create a new project directory
    let is_project_name_valid = is_project_name_valid(&project_name);
    if !is_project_name_valid.0 {
        println!("{}", is_project_name_valid.1);
        return;
    }
    // Prompt user for project description
    let project_description = prompt_for_project_description();
    create_new_project_directory(&project_name);

    // copy the default template to the project directory
    copy_default_template(&project_name);

    // create a toml file called _config.toml
    create_config_file(&project_name, &project_description);

    println!("🚀🚀 Project created successfully! 🚀🚀");
    println!("Run `cd {}` to enter the project directory", project_name);
    println!("Run `plainshare build .` to build the project");
    println!("Run `plainshare dev .` to start the development server");
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
    println!("📝 Enter a description for your project:");
    std::io::stdin()
        .read_line(&mut project_description)
        .expect("Failed to read line");
    // escape quotes including single quotes
    project_description = project_description.replace("\"", "\\\"");
    // escape new lines
    project_description = project_description.replace("\n", "");
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

fn copy_default_template(project_name: &String) {
    let options = CopyOptions::new(); //Initialize default values for CopyOptions
    // copy dir1 and file1.txt to target/dir1 and target/file1.txt
    let mut from_paths = Vec::new();
    from_paths.push("src/default_template");
    copy_items(&from_paths, &project_name, &options)
        .expect("🙅‍♂️ Failed to copy default template.");
    // rename the default_template to src
    let src_dir = std::path::Path::new(&project_name).join("default_template");
    let dest_dir = std::path::Path::new(&project_name).join("src");
    std::fs::rename(src_dir, dest_dir).expect("🙅‍♂️ Failed to rename default_template to src.");
}
