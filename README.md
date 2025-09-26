# About
Terminal application written in Rust that displays the total amount of time spent watching anime by fetching data from [anilist](https://anilist.co/).

# Screenshots
| [anijouhou](https://github.com/legendofmaj/anijouhou/releases) | [anijouhou with fastfetch](https://github.com/fastfetch-cli/fastfetch) |
| :-----------------------------------------------------------:  | :--------------------------------------------------------------------: |
| <img src="res/anijouhou.png" width="400"/>                     | <img src="res/fastfetch_anijouhou.png" width="400"/>                   |

# Usage
- Basic usage: `anijouhou`
- Delete user data directory: `anijouhou -d` or `anijouhou --delete`
- Get only total watchtime hours: `anijouhou -h` or `anijouhou --hours`
- Get only total watchtime in minutes: `anijouhou -m` or `anijouhou --minutes`
- Get only total amount of episodes watched: `anijouhou -e` or `anijouhou --episodes`

# Installation
Download the latest release from the [release page](https://github.com/legendofmaj/anijouhou/releases) <br>
Copy it to your bin directory
```
sudo cp anijouhou /usr/bin/
```

# Build from source
Clone the repository
```
git clone https://github.com/legendofmaj/anijouhou.git && cd anijouhou
```
Build the project
```
cargo build --release
```
Copy the binary to your bin directory
```
sudo cp target/release/anijouhou /usr/bin/
```
