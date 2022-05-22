#![allow(unused)]

use clap::Parser;
use std::path::Path;
use std::fs;

/// Create a quick and easy Twitter and Discord compliant page to embed videos in those platforms. Make sure the video file and the resulting .html file are both in the url location you specifiy
#[derive(Parser)]
struct Cli {
    /// The path to video file
    path: String,

    /// The html file to output
    #[clap(short, long, default_value = "./index.html")]
    out: String,

    /// The height of the video in pixels
    #[clap(short, long)]
    height: u32,

    /// The width of the video in pixels
    #[clap(short, long)]
    width: u32,

    /// The title for the page
    #[clap(short, long, default_value = "")]
    page_title: String,

    /// The description for the video
    #[clap(short, long, default_value = "")]
    video_description: String,

    /// The url where the video will be stored (with trailing /)
    #[clap(short, long, default_value = "https://buymymojo.net/Video/")]
    url: String,

}

const html_part1: &str = "<!DOCTYPE html>
<html lang=\"en\" prefix=\"og: http://ogp.me/ns#\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>";
    
    const html_part2: &str = "</title>
    <style>
        body {
            background-color: #000;
            color: #fff;
        }
        .container {
            width: 70%;
            height: 70%;
            display: flex;
            justify-content: center;
            align-items: center;
            align-self: center;
        }
        .container video {
            width: 100%;
            height: 100%;
            align-self: center;
        }
    </style>

    <meta property=\"og:title\" content=\""; 

    const html_part3: &str = "\">
    <meta property=\"og:description\" content=\"";

    const html_part4: &str = "\">
    <meta property=\"og:type\" content=\"video.other\">
    <meta property=\"og:video:url\" content=\"";

    const html_part5: &str = "\">
    <meta property=\"og:video:type\" content=\"video/";

    const html_part6: &str = "\">
    <meta property=\"og:video:width\" content=\"";

    const html_part7: &str = "\">
    <meta property=\"og:video:height\" content=\"";

    const html_part8: &str = "\">
    
    <meta name=\"twitter:card\" content=\"player\">";

    const html_part9: &str = "
    <meta name=\"twitter:title\" content=\"";

    const html_part10: &str = "\">
    <meta name=\"twitter:player\" content=\"";

    const html_part11: &str = "\">
    <meta name=\"twitter:player:height\" content=\"";

    const html_part12: &str = "\">
    <meta name=\"twitter:player:width\" content=\"";

    const html_part13: &str = "\">
    <meta name=\"twitter:description\" content=\"";

    const html_part14: &str = "\">
    </head>
    <body>
        <center>
            <div class=\"container\">
                <video controls preload=\"auto\">
                    <source src=\"";
    
    const html_part15: &str = "\" type=\"video/";

    const html_part16: &str = "\">
            </video>
        </div>
    </center>
</body>
</html>";

fn main() {
    let args = Cli::parse();

    let input_file = Path::new(&args.path);
    let file_type = input_file.extension().unwrap().to_str().unwrap();
    let file_name = input_file.file_name().unwrap().to_str().unwrap();
    let video_url = format!("{}{}", args.url, file_name);
    let output_path = Path::new(&args.out);
    let out_file_name = output_path.file_name().unwrap().to_str().unwrap();

    let mut page_title = format_title(args.page_title, file_name);
    let mut video_description = &format_description(args.video_description, file_name);

    let html_page = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", html_part1, page_title, html_part2, page_title, html_part3, video_description, html_part4, video_url, html_part5, file_type, html_part6, args.width, html_part7, args.height, html_part8, html_part9, page_title, html_part10, video_url, html_part11, args.height, html_part12, args.width, html_part13, video_description, html_part14, video_url, html_part15, file_type, html_part16);

    fs::write(&args.out, html_page).expect("Unable to write file, make sure you include the file name and .html with -o");

    println!("Finished writing {}", out_file_name);
    print!("Make sure both the video file and {} are both in {}", out_file_name, args.url)
}

fn format_description(desc: String, file_name: &str) -> String
{
    if desc != ""{
        return desc;
    } else {
        return format!("Video: {}", file_name);
    }
}

fn format_title(title: String, file_name: &str) -> String
{
    if title != ""{
        return title;
    } else {
        return file_name.to_string();
    }
}
