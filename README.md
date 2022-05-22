# Video Web Page Generator

A basic tool written in rust that makes a html page for embeding videos in discord and twitter from other websites.

## Badges

![Crates.io](https://img.shields.io/crates/v/video_web_page_generator)
![Crates.io](https://img.shields.io/crates/l/video_web_page_generator)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/BuyMyMojo/video-web-page-generator/Rust)

## Usage

```bash
Create a quick and easy Twitter and Discord compliant page to embed videos in those platforms. Make
sure the video file and the resulting .html file are both in the url location you specifiy

USAGE:
    video_web_page_generator.exe [OPTIONS] --height <HEIGHT> --width <WIDTH> <PATH>

ARGS:
    <PATH>    The path to video file

OPTIONS:
    -h, --height <HEIGHT>
            The height of the video in pixels

        --help
            Print help information

    -o, --out <OUT>
            The html file to output [default: ./index.html]

    -p, --page-title <PAGE_TITLE>
            The title for the page [default: ]

    -u, --url <URL>
            The url where the video will be stored (with trailing /) [default:
            https://buymymojo.net/Video/]

    -v, --video-description <VIDEO_DESCRIPTION>
            The description for the video [default: ]

    -w, --width <WIDTH>
            The width of the video in pixels
```

## Installation

install with just cargo:

```bash
  cargo install video_web_page_generator
```

Install from github with cargo:

```bash
  git clone https://github.com/BuyMyMojo/video-web-page-generator.git
  cd video-web-page-generator
  cargo install --path ./
```
