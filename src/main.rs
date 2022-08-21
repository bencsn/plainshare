use clap::{Parser, Subcommand};
mod builder;
mod project;
mod utils;
use rocket::fairing::AdHoc;
use rocket::response::content::RawHtml;
use rocket::{get, routes, Request};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a development server
    Dev {
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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::New { project_name } => {
            project::new(project_name);
        }
        Commands::Build { project_path } => {
            let valid_paths = builder::build(project_path.unwrap_or('.'.to_string()));
            println!("👀 Valid paths: {:?}", valid_paths);
        }
        Commands::Dev { project_path } => {
            let project_path_str = project_path.unwrap_or('.'.to_string());
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
                .mount("/", routes![index, page])
                .launch()
                .await?;
        }
    };

    Ok(())
}

#[get("/<name>")]
fn page(name: &str) -> RawHtml<std::string::String> {
    println!("👀 Serving: {}", name);
    let build_path = "testprojct/build";
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
fn index() -> RawHtml<std::string::String> {
    let build_path = "testprojct/build";
    let index_path = format!("{}/index.html", build_path);
    let index = std::fs::read_to_string(&index_path).unwrap_or_default();
    if index.is_empty() {
        println!("No index.html file found in {}", index_path);
    }
    let raw_html = RawHtml(index);
    raw_html
}
