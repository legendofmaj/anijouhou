# About
Terminal application written in Rust that displays the total amount of time spent watching anime by fetching data from [anilist](https://anilist.co/).

# Screenshots
| [anijouhou](https://github.com/legendofmaj/anijouhou/releases) | [anijouhou with fastfetch](https://github.com/fastfetch-cli/fastfetch) |
| :-----------------------------------------------------------:  | :--------------------------------------------------------------------: |
| <img src="res/anijouhou.png" width="500"/>                     | <img src="res/fastfetch_anijouhou.png" width="500"/>                   |

# Usage
- Basic usage: `anijouhou`
## File management
- Clear cache (automically cleared daily): `anijouhou -c` or `anijouhou --clear-cache`
- Delete user data directory (`$HOME/.config/anijouhou`): `anijouhou -d` or `anijouhou --delete`
## Output formatting
- Get only total watchtime hours: `anijouhou -h` or `anijouhou --hours`
- Get only total watchtime in minutes: `anijouhou -m` or `anijouhou --minutes`
- Get only total amount of episodes watched: `anijouhou -e` or `anijouhou --episodes`
## Supplying user data via command line arguments
- Give username via command line argument: `anijouhou -u <your-username>` or `anijouhou --username <your-username>`
- Give api-key via command line argument: `anijouhou -k <api-key>` or `anijouhou --api-key <api-key>`
>[!Tip]
> If you give `skip` as the api-key, none will be used.

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

# Tips and tricks
## Use anijouhou in fastfetch
Simply add the following to your `~/.config/fastfetch/config.jsonc`
```jsonc 
{
  "type": "command",
  "text": "anijouhou -h" // or any other flag you want
}
```
## Switch between accounts
`anijouhou` does not directly provide a way to switch between anilist accounts. However you can write a shell script like the one below to get this functionality. <br>
See [here](scripts/README.md) for additional information on the script below.
```bash
#!/bin/bash
# In this example user1 requires an api-key, while user2 does not.
username1="your_username"
api_key_user1="your_api_key"

anijouhou -d
if [[ $1 == "$username1" ]];
then
  anijouhou -u "$1" -k $api_key_user1
else
  anijouhou -u "$1" -k skip
fi
```
