# About
Terminal application written in Rust that displays the total amount of time spent watching anime by fetching data from [anilist](https://anilist.co/).

# Screenshot
<img height="375" alt="anijouhou running in kitty." src="res/anijouhou.png" />

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
