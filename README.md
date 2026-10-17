# choicecompiler &emsp; <img alt="last updated" src="https://img.shields.io/date/0?style=for-the-badge&label=last%20updated&color=71bbc1" height="24"> [<img alt="github" src="https://img.shields.io/badge/github-reezey/choicecompiler-5b969b?style=for-the-badge&labelColor=555555&logo=github" height="24">](https://github.com/ReeZey/choicecompiler) <img alt="open issues" src="https://img.shields.io/github/issues/reezey/choicecompiler?style=for-the-badge&color=%233d6467" height="24"> <img alt="serde" src="https://img.shields.io/deps-rs/serde/latest?style=for-the-badge&label=serde&color=rgb(10%2C%20180%2C%2030" height="24"> 

This is a binary that can be used to create `.mp4` files from raw `ChoicerVoicer` files you get when you save them (`Save Dub`).

This can be used to easily share on discord etc.

## How to setup

### Download

Go to [`releases`](https://github.com/ReeZey/choicecompiler/releases) and download the binary for your operating system.

### FFmpeg

In order to use this binary, you'll need `ffmpeg` in your system `$PATH`.  
Make sure `ffmpeg` is installed before continuing.

### Config

The only configurable setting is `game_path` which is the directory where the game saves data.

```toml
# Default config
game_path = "%appdata%/YeahMaybe/ChoicerVoicer/game"
```

> [!IMPORTANT]  
> You will need to change this config if you're not running Windows.

## How to use

1. Run the binary
    ```sh
    choicecompiler.exe
    ```
> [!NOTE]  
> On other operating systems, this will be different.  

2. Select a *dub pack* from the list by it's number
    ```sh
    0. <name>
    1. <name>
    2. <name>
    3. <name>
    ```
3. Select a date  
    This is which recording will be used.
    ```sh  
    0. <date>
    1. <date>
    2. <date>
    3. <date>
    ```
4. Type in a output name
    ```sh
    my_file
    ```
> [!CAUTION]  
> File name must not end with `.mp4`  

5. Let FFmpeg run wild
6. ???
7. ***Profit***

## Issues?

Look at [`contributing.md`](/contributing.md)