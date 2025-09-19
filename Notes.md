# Getting to run the example file
- Create the necessary files
````
cargo new project-name
````
- Add the necessary dependencies to your `cargo.toml`
````
[dependencies]
serde_json = "1.0"
reqwest = "0.11.8"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
````
- Run the project
`cargo run`
- run a specific file
`cargo run --bin file_name`

# Getting the total watched time for all anime
See https://docs.anilist.co/reference/object/userstats#userstats for reference

# Authorization
