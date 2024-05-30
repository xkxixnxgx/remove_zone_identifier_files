# Removing *Zone.Identifier on WSL

## Description 
### Script remove additional *:Zone.Identifier files which Windows creates when You copy files from Windows to WSL2 storage.

### We can clean it.

## Bootstrap
#### You need to have:
- `linux terminal`
- `connection to github by ssh` (optional)
- `make`(optional)
- `installed Rust` (optional)
- `good mood` :)

## How use

### 1. Simple way

#### Load `main` file by [link](https://github.com/xkxixnxgx/remove_zone_identifier_files/raw/main/main)
#### Copy `main` file to your directory for cleaning
#### Run `main` file in linux (WSL2) terminal by this command
```bash
./main
```
#### Don't forget to remove `main` file

### 2. Way through compiling
#### Clone repo
```bash
git clone git@github.com:xkxixnxgx/remove_zone_identifier_files.git
```
#### Recompile `main` file
```bash
make compile
```
#### Copy `main` file to your directory for cleaning
#### Run `main` file in linux (WSL2) terminal by this command
```bash
./main
```

## Thanks!
