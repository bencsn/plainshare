use markdown::to_html;
use serde::Deserialize;

use crate::utils;

#[derive(Debug, Deserialize)]
struct Config {
    build_target: String,
}

fn read_config(toml_path: &String) -> std::io::Result<Config> {
    let content = std::fs::read_to_string(toml_path)?;
    Ok(toml::from_str(&content)?)
}

/// Returns a vector of all valid route paths
pub fn build(project_path: String) -> Vec<std::string::String> {
    println!("Building project");
    let config_path = project_path.to_owned() + "/_config.toml";
    // read _config.toml file
    let content = match read_config(&config_path) {
        Ok(content) => content,
        Err(error) => {
            panic!("🙅‍♂️ Failed to read _config.toml file: {}", error);
        }
    };

    println!("Build target: {}", content.build_target);
    if content.build_target.is_empty() {
        panic!("🙅‍♂️ Build target not specified in _config.toml");
    }

    let build_path = project_path.to_owned() + "/" + &content.build_target;
    let routes_path = project_path.to_owned() + "/routes";

    // create a directory build_path
    // if build directory already exists, delete it
    let build_dir = std::path::Path::new(&build_path);
    if build_dir.exists() {
        std::fs::remove_dir_all(build_dir).expect("🙅‍♂️ Failed to delete build directory.");
    }

    std::fs::create_dir(build_dir).expect("🙅‍♂️ Failed to create build directory.");

    let routes_path_as_path = std::path::Path::new(&routes_path);
    let mut visited_paths: Vec<String> = Vec::new();
    let _result = match utils::visit_dirs(routes_path_as_path, &mut visited_paths) {
        Ok(result) => result,
        Err(error) => {
            panic!("🙅‍♂️ Failed to read routes directory: {}", error);
        }
    };

    print!("👀 Visited paths: {:?}", visited_paths);

    let mut valid_route_strings: Vec<String> = Vec::new();
    for route_string in visited_paths {
        let route_path = std::path::Path::new(&route_string);
        if route_string.ends_with(".md") {
            // read route content as string
            let route_content = match std::fs::read_to_string(route_path) {
                Ok(content) => content,
                Err(error) => {
                    panic!("🙅‍♂️ Failed to read route content: {}", error);
                }
            };
            println!("Route: {:?}", route_content);
            let html_result = to_html(&route_content);
            println!("HTML: {:?}", html_result);
            let html_path_string = route_string
                .replace(&routes_path, &build_path)
                .replace(".md", ".html");
            let html_path = std::path::Path::new(&html_path_string);
            println!("HTML path: {:?}", html_path_string);
            std::fs::create_dir_all(html_path.clone().parent().unwrap())
                .expect("🙅‍♂️ Failed to create directory for HTML file.");
            match std::fs::write(&html_path_string, html_result) {
                Ok(_) => {
                    println!("👍 HTML file written");
                    valid_route_strings.push(html_path_string);
                }
                Err(error) => {
                    panic!("🙅‍♂️ Failed to write HTML file: {}", error);
                }
            };
        }
    }

    return valid_route_strings;
}
