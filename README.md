# About
anijouhou (アニメ情報) is a terminal application that displays the total amount of time spent watching anime by fetching data from [anilist](https://anilist.co/).

# Screenshots
| [anijouhou](https://github.com/legendofmaj/anijouhou/releases) | [anijouhou with fastfetch](https://github.com/fastfetch-cli/fastfetch) |
| :-----------------------------------------------------------:  | :--------------------------------------------------------------------: |
| <img src="res/anijouhou.png" width="500"/>                     | <img src="res/fastfetch_anijouhou.png" width="500"/>                   |

# Installation
```bash
cargo install --git "https://github.com/legendofmaj/anijouhou.git"
```
>[!Note]
> Note that this requires cargo to be installed.

Should you not have `cargo` installed, or want to simply download the binary, you can do so from the [release page](https://github.com/legendofmaj/anijouhou/releases) or get the latest development version from [GitHub actions](https://github.com/legendofmaj/anijouhou/actions).

Binaries are available for both Linux and Windows. In order to launch the application without being in the directory the binary is located in you will have to add it to your PATH.

On Linux you can this as follows:
```
sudo cp anijouhou /usr/bin/
```
On Windows the process is a bit more involved. You can read about it [here](https://stackoverflow.com/questions/9546324/adding-a-directory-to-the-path-environment-variable-in-windows).

# Build from source
Clone the repository and go into the directory.
```bash
git clone https://github.com/legendofmaj/anijouhou.git && cd anijouhou
```
Build the project.
```bash
cargo build --release
```
Copy the binary to a directory in your PATH.
```bash
cp target/release/anijouhou $HOME/.cargo/bin/
```

# Usage
## Basic usage
```bash
anijouhou <username>
```
If you want to get information about the same profile again, you can omit `<username>`.

## File management
Clear cache (automatically cleared daily): 
```bash
anijouhou -c
# or
anijouhou --clear-cache
```
Delete user data directory (`$HOME/.config/anijouhou/` or `%APPDATA%\anijouhou\`): 
```bash
anijouhou -d 
# or
anijouhou --delete
```
## Output formatting
Get total watchtime in:
```bash
anijouhou -h # hours
anijouhou -m # minutes
anijouhou -e # episodes
```
Or alternatively:
```bash
anijouhou --hours
anijouhou --minutes
anijouhou --episodes
```
## Supplying user data via command line arguments
>[!TIP]
> If no flag is passed, but an argument, anijouhou assumes since  `v0.3.5` that the argument is a username of a public user.
> This means that `anijouhou -u <your-username> -k skip` is equivalent to `anijouhou <your-username>`.

Give username via command line argument:
```bash
anijouhou -u <your-username> 
# or 
anijouhou --username <your-username>
```
Give api key via command line argument: 
```bash
anijouhou -k <api-key> 
# or 
anijouhou --api-key <api-key>
```
>[!Important]
> If you give `skip` as the api key, none will be used.

# Tips and tricks
## Use anijouhou in fastfetch
Add the following to your `~/.config/fastfetch/config.jsonc`
```jsonc 
{
  "type": "command",
  "text": "anijouhou -h" // or any other flag you want
}
```
## Switch between accounts
Should you want to switch between multiple private accounts, you can do so with a shell script like the one shown below.
```bash
#!/bin/bash
username_1="your_username"
api_key_1="your_api_key"

username2="your_second_username"
api_key_2="your_second_api_key"

if [[ $1 == "$username_1" ]];
then
  anijouhou "$username_1" -k "$api_key_1"
else
  anijouhou "$username_2" -k "$api_key_2"
fi
```
To run the script: 
```bash
chmod +x script_name.sh && ./script_name.sh
```
>[!Important]
> This script requires `bash` to be installed. It does not work on Windows, unless you use WSL.
## Use anijouhou on Android via Termux
Install necessary dependencies
```bash
pkg install rust openssl
```
Install anijouhou
```bash
cargo install --git "https://github.com/legendofmaj/anijouhou.git"
```