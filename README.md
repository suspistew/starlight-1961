<p align="center">
  <img width="704" height="148" src="https://github.com/grzi/starlight-1961/blob/main/assets/ui/logo.png?raw=true">
</p>

**Welcome, to Starlight 1961 !**

In his speech in september 1962, JFK told 
> "Within these last 19 months at least 45 satellites have circled the earth"

You take place in the space ship named "Starlight 1961", that delivers things and stuff to some of these satellites. 
Will you be able to land on each one of them ?

Have fun !

## Contributing

Contributions are welcome. 

### How to build

**Linux** users may install some dependencies so as to compile the project
```bash
sudo apt install libasound2-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**MacOs** users have to install xcode.


To build the project, Windows and Linux users has to explicitly choose `"vulkan"` with the following command:
```bash
cargo build --features "vulkan"
```
MacOs users may explicitly choose `"metal"` with the following command:

```bash
cargo build --features "vulkan"
```
### How to run

Windows and Linux users has to explicitly choose `"vulkan"` with the following command:

```bash
cargo run --features "vulkan"
```

MacOs users may explicitly choose `"metal"` with the following command:

```bash
cargo run --features "metal"
```

### Credits 
