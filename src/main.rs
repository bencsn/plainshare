use clap::{Parser, Subcommand};
mod builder;
mod generator;
mod utils;
use rocket::response::content::RawHtml;
use rocket::{get, routes, State};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a server
    Start {
        #[clap(value_parser)]
        project_path: Option<String>,
    },

    /// Build the project
    Build {
        #[clap(value_parser)]
        project_path: Option<String>,
    },

    /// Create a new project
    New {
        #[clap(value_parser)]
        project_name: String,
    },
}

struct ProjectInfo {
    project_path: String,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::New { project_name } => {
            generator::new(project_name);
        }
        Commands::Build { project_path } => {
            let current_dir = std::env::current_dir().unwrap();
            let current_dir_string = current_dir.to_str().unwrap().to_string();
            let valid_paths = builder::build(project_path.unwrap_or(current_dir_string));
            println!("👀 Valid paths: {:?}", valid_paths);
        }
        Commands::Start { project_path } => {
            let current_dir = std::env::current_dir().unwrap();
            let current_dir_string = current_dir.to_str().unwrap().to_string();
            let project_path_str = project_path.unwrap_or(current_dir_string);
            let valid_paths_including_build_path = builder::build(project_path_str.to_owned());
            // TODO: Start a dev server
            println!(
                "Preparing to start development server... {:?}",
                valid_paths_including_build_path
            );

            // extract real paths from valid_paths, excluding anything before /build/
            let mut paths_excluding_build_path: Vec<String> = Vec::new();
            for path in valid_paths_including_build_path {
                let path_as_path = std::path::Path::new(&path);
                let build_path_string = project_path_str.to_owned() + "/build";
                // remove build_path_string from path
                let real_path = path_as_path
                    .strip_prefix(build_path_string)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                paths_excluding_build_path.push(real_path);
            }

            println!("👀 Real paths: {:?}", paths_excluding_build_path);
            let mut _rocket = rocket::build()
                .manage(ProjectInfo {
                    project_path: project_path_str,
                })
                .mount("/", routes![index, page])
                .launch()
                .await?;
        }
    };

    Ok(())
}

#[get("/<name>")]
fn page(name: &str, state: &State<ProjectInfo>) -> RawHtml<std::string::String> {
    println!("👀 Serving: {}", name);
    let project_path = &state.project_path;
    if project_path.is_empty() {
        println!("No project path found");
        return RawHtml("500".to_string());
    }
    let build_path = format!("{}/build", project_path);
    let html_file_path = format!("{}/{}.html", build_path, name);
    let index = std::fs::read_to_string(&html_file_path).unwrap_or_default();
    if index.is_empty() {
        println!("No index.html file found in {}", html_file_path);
        println!("Trying adding index.html to the end of the path");
        let html_file_path = format!("{}/{}/index.html", build_path, name);
        let index = std::fs::read_to_string(&html_file_path).unwrap_or_default();
        if index.is_empty() {
            println!("No index.html file found in {}", html_file_path);
            return RawHtml("404".to_string());
        }

        return RawHtml(index);
    }
    let raw_html = RawHtml(index);
    raw_html
}

#[get("/")]
fn index(state: &State<ProjectInfo>) -> RawHtml<std::string::String> {
    let project_path = &state.project_path;
    if project_path.is_empty() {
        println!("No project path found");
        return RawHtml("500".to_string());
    }
    let build_path = format!("{}/build", project_path);
    let index_path = format!("{}/index.html", build_path);
    let index = std::fs::read_to_string(&index_path).unwrap_or_default();
    if index.is_empty() {
        println!("No index.html file found in {}", index_path);
    }
    let raw_html = RawHtml(index);
    raw_html
}
