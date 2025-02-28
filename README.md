# buddy

**Buddy** is here to keep you company while you work on your computer! It’s an animated, interactive little friend that runs across your screen and responds to your clicks.

![](res/example.gif)

## Prerequisites 
- **Rust**: Make sure you have Rust installed. If not, head over to [Rust’s official website](https://www.rust-lang.org/tools/install) for installation instructions.
- **GTK4**: Buddy uses GTK4 for rendering the character. It can be installed on Linux by running:

```bash
sudo apt install libgtk-4-dev
```

## Installation 
1. Clone the repository to your local machine:

```bash
git clone https://github.com/hannahfluch/buddy.git
cd buddy
```

2. Build the project using cargo
```bash
cargo build --release
```

## How to Run 🏃

Run this command to use buddy with the rat sprites, a width of 200 pixels, 9fps and a movement speed of 35:
```bash
buddy -s ./res/rat_sprites/ -w 200 -f 9 -m 35
```

Buddy is also able to infer the width/height according to the aspect ratio of the original picture, if only one dimension is provided.
> Note: This can lead to unwanted behavior when switching sprites on-the-fly.

For more information run this command:
```bash
buddy -h
```

## Reloading Sprites On-the-Fly
Want to update Buddy's appearance without restarting the program? Buddy can receive signals to reload the sprites:

```bash
kill -SIGUSR1 <pid>
```
> Send SIGUSR1 or SIGUSR2

Replace <pid> with the process ID of the Buddy instance. This will trigger Buddy to reload the sprite animations dynamically

Instead, the `automatic-reload` flag in combination with the `signal-frequency` configuration can be used.

## Configuration ⚙️
Buddy creates a default configuration file upon its first run. This file is located at:

```bash
~/.config/buddy/config.toml
```

### Default Configuration File
The configuration file includes all necessary settings to customize your Buddy's behavior and appearance. However, for Buddy to function, **you must specify a valid sprite path**:
1. Using **command-line arguments**
2. Adding the sprite path to the **configuration file**

## Custom Sprites 🎨
Buddy thrives on customization! Just provide a directory containing 3 different gifs(`idle`, `click`, `run`), and watch your Buddy come to life with your own animations.
