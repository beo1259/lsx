extern crate directories;
use std::{
    collections::HashMap,
    env::current_dir,
    fs,
    hash::Hash,
    path::{Path, PathBuf},
    process::Output, str::SplitAsciiWhitespace,
};

use clap::command;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use fs_extra::dir::{self, get_size};

extern crate fs2;

extern crate human_bytes;
use human_bytes::human_bytes;

extern crate sysinfo;
use sysinfo::{Components, Disk, Disks, System};

use colored::{Color, ColoredString, Colorize};

use clap::Parser;

/// A more versatile 'ls', for any and every CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 'lsx -s' to show sizes of each item listed.
    #[arg(short, long)]
    show_sizes: bool,
}

fn get_disk() -> PathBuf {
    let disks: Disks = Disks::new_with_refreshed_list();
    let mut dir_disk = PathBuf::from("placeholder");

    for disk in &disks {
        let cur_disk = PathBuf::from(disk.mount_point().to_str().unwrap());
        let cur_dir_string = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();

        if cur_dir_string.starts_with(&cur_disk.to_string_lossy().to_string()) {
            dir_disk = cur_disk.to_path_buf();
        }
    }

    let dir_s = dir_disk.to_string_lossy().to_string();
    let dir_slice = &dir_s[0..1];
    PathBuf::from(dir_slice)
}

fn visual_usage(dir: &PathBuf) -> String {
    let cur = current_dir().unwrap();
    let dir_size = get_size(cur).unwrap() as f64;

    let size_total_raw = fs2::total_space(dir).unwrap() as f64;

    let left_int = size_total_raw as u64;

    let boxes_to_cover = (dir_size / size_total_raw) * 20.0;
    let boxes_to_cover_int = boxes_to_cover.round() as u64;

    let mut du_visual: &str = "▯▯▯▯▯▯▯▯▯▯▯▯▯▯▯▯▯▯▯▯";
    let mut count = 0;

    let formatted: String = du_visual
        .chars()
        .map(|c| {
            if count < boxes_to_cover_int {
                count += 1;
                "▮"
            } else {
                "▯"
            }
        })
        .collect();

    formatted
}

fn dir_used(dir: &PathBuf) -> f64 {
    get_size(dir).unwrap() as f64
}

fn used_readable(dir: &PathBuf) -> String {
    human_bytes(dir_used(dir))
}

fn get_type_color(ext: &str) -> ColoredString {
    let mime_g = mime_guess::from_ext(ext).first_or_text_plain().to_string();

    let mime = match mime_g.as_str() {
        "application/pdf" | "application/x-pdf" => ext.truecolor(252, 73, 61),

        "application/vnd.rar"
        | "application/x-rar-compressed"
        | "application/zip"
        | "application/x-zip-compressed"
        | "multipart/x-zip"
        | "application/vnd.cncf.helm.chart.content.v1.tar+gzip" => ext.bright_yellow(),

        "application/vnd.ms-powerpoint"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        | "application/vnd.openxmlformats-officedocument.presentationml.template"
        | "application/vnd.openxmlformats-officedocument.presentationml.slideshow"
        | "application/vnd.ms-powerpoint.addin.macroEnabled.12"
        | "application/vnd.ms-powerpoint.presentation.macroEnabled.12"
        | "application/vnd.ms-powerpoint.template.macroEnabled.12"
        | "application/vnd.ms-powerpoint.slideshow.macroEnabled.12" => ext.truecolor(235, 111, 16),

        "application/vnd.ms-excel"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.template"
        | "application/vnd.ms-excel.sheet.macroEnabled.12"
        | "application/vnd.ms-excel.template.macroEnabled.12"
        | "application/vnd.ms-excel.addin.macroEnabled.12"
        | "application/vnd.ms-excel.sheet.binary.macroEnabled.12" => ext.truecolor(45, 117, 30),

        "application/msword"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.template"
        | "application/vnd.ms-word.document.macroEnabled.12"
        | "application/vnd.ms-word.template.macroEnabled.12" => ext.truecolor(30, 60, 117),

        "text/plain" => ext.white(),
        _ => ext.white(),
    };

    let mime = match mime_g.as_str() {
        "application/pdf" | "application/x-pdf" => ext.truecolor(230, 25, 25),

        "application/vnd.rar"
        | "application/x-rar-compressed"
        | "application/zip"
        | "application/x-zip-compressed"
        | "multipart/x-zip"
        | "application/vnd.cncf.helm.chart.content.v1.tar+gzip" => ext.truecolor(179, 43, 149),

        "application/vnd.ms-powerpoint"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        | "application/vnd.openxmlformats-officedocument.presentationml.template"
        | "application/vnd.openxmlformats-officedocument.presentationml.slideshow"
        | "application/vnd.ms-powerpoint.addin.macroEnabled.12"
        | "application/vnd.ms-powerpoint.presentation.macroEnabled.12"
        | "application/vnd.ms-powerpoint.template.macroEnabled.12"
        | "application/vnd.ms-powerpoint.slideshow.macroEnabled.12" => ext.truecolor(235, 111, 16),

        "application/vnd.ms-excel"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.template"
        | "application/vnd.ms-excel.sheet.macroEnabled.12"
        | "application/vnd.ms-excel.template.macroEnabled.12"
        | "application/vnd.ms-excel.addin.macroEnabled.12"
        | "application/vnd.ms-excel.sheet.binary.macroEnabled.12" => ext.truecolor(51, 135, 58),

        "application/msword"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.template"
        | "application/vnd.ms-word.document.macroEnabled.12"
        | "application/vnd.ms-word.template.macroEnabled.12" => ext.truecolor(73, 67, 232),

        _ if mime_g.as_str().starts_with("audio") => ext.truecolor(25, 230, 203),
        _ if mime_g.as_str().starts_with("video") => ext.truecolor(134, 25, 230),
        _ if mime_g.as_str().starts_with("image") => ext.truecolor(237, 208, 17),
        _ if mime_g.as_str().starts_with("application") => ext.truecolor(250, 206, 145),

        "text/plain" => ext.white(),
        _ => ext.white(),
    };

    mime
}

fn print_dirs_and_size() {
    let cur = current_dir().unwrap();

    let paths = fs::read_dir(&cur).unwrap();

    let disk = get_disk();

    let mut longest_str: String = "".to_string();

    let paths_for_len = fs::read_dir(&cur).unwrap();

    for path in paths_for_len {
        let cur_path = &path.unwrap().path();
        let cur_fn = cur_path.file_name().unwrap().to_string_lossy().to_string();

        if cur_fn.len() > longest_str.len() {
            longest_str = cur_fn
        }
    }

    let spaces_amt = longest_str.len();
    let mut spaces_global: usize = spaces_amt;
    let mut output: Vec<String> = Vec::new();

    let mut counter = 0;
    for path in paths {
        let cur_path = &path.unwrap().path();

        let spaces_to_add = if cur_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .len()
            < longest_str.len()
        {
            let final_amt = spaces_amt
                - cur_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .len();
            " ".repeat(final_amt)
        } else {
            " ".to_string()
        };

        if cur_path.is_dir() {
            output.push(format!(
                "{}{} {} {}",
                "/".white(),
                &cur_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .yellow()
                    .underline(),
                    "-",
                used_readable(&cur_path).truecolor(38, 132, 255)
            ));
        }
        
         else {
            let ext = Path::new(&cur_path).extension();
            let mut colored_ext = "".red();
            let mut found_ext: bool = false;

            match ext {
                Some(e) => {
                    found_ext = true;
                    colored_ext = get_type_color(&e.to_str().unwrap())
                }
                None => colored_ext = "".white(),
            }


            output.push(format!(
                "{}.{} {} {}",
                &cur_path.file_stem().unwrap()
                    .to_string_lossy()
                    .to_string()
                    .truecolor(222, 222, 222),
                &colored_ext,
                "-",
                used_readable(&cur_path).truecolor(38, 132, 255)
            ));
        }

        counter += 1;
    }
    println!();

    let halfway_point = output.len() / 2;
    let (left_column, right_column) = output.split_at(halfway_point);
    
    let longest_left = left_column.iter().map(|s| s.len()).max().unwrap();
    let longest_right = right_column.iter().map(|s| s.len()).max().unwrap();
    
    for (left, right) in left_column.iter().zip(right_column.iter()) {
        let left_padding = " ".repeat(longest_left - left.len());
        let right_padding = " ".repeat(longest_right - right.len());
    
        println!("{left}{left_padding}    {right}{right_padding}");
    }
    // If there are an odd number of items, print the last item in left column
    if left_column.len() > right_column.len() {
        println!("{}", left_column.last().unwrap());
    }

    println!();

    return;
}



fn print_dirs() {
    let cur = current_dir().unwrap();

    let paths = fs::read_dir(&cur).unwrap();

    let disk = get_disk();

    let mut longest_str: String = "".to_string();

    let paths_for_len = fs::read_dir(&cur).unwrap();

    let mut output: Vec<String> = Vec::new();

    let mut counter = 0;

    for path in paths_for_len{
        let cur = path.unwrap().file_name().to_string_lossy().to_string();

        if cur.len() > longest_str.len(){
            longest_str = cur;
        }
    }

    for path in paths {
        let cur_path = &path.unwrap().path();

        if cur_path.is_dir() {
            output.push(format!(
                "{}{}",
                "/".white(),
                &cur_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .yellow()
                    .underline()
            ));
        } else {
            //let stem = cur_path.file_stem().unwrap().to_string_lossy().to_string();
            let ext = Path::new(&cur_path).extension();
            let mut colored_ext = "".red();
            let mut found_ext: bool = false;

            match ext {
                Some(e) => {
                    found_ext = true;
                    colored_ext = get_type_color(&e.to_str().unwrap())
                }
                None => colored_ext = "".white(),
            }


            if found_ext {
                output.push(format!(
                    "{}.{}",
                    &cur_path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .truecolor(222, 222, 222),
                    colored_ext
                ));
            } else {
                output.push(format!(
                    "{}",
                    &cur_path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .truecolor(212, 241, 250)
                ));
            }
        }

        counter += 1;
    }

    println!();

    let halfway_point = output.len() / 2;
    let (left_column, right_column) = output.split_at(halfway_point);
    
    let longest_left = left_column.iter().map(|s| s.len()).max().unwrap_or(0);
    let longest_right = right_column.iter().map(|s| s.len()).max().unwrap_or(0);
    
    for (left, right) in left_column.iter().zip(right_column.iter()) {
        let left_padding = " ".repeat(longest_left - left.len());
        let right_padding = " ".repeat(longest_right - right.len());
    
        println!("{left}{left_padding}   {right}{right_padding}");
    }
    // If there are an odd number of items, print the last item in left column
    if left_column.len() > right_column.len() {
        println!("{}", left_column.last().unwrap());
    }
    

    println!();

    return;
}

fn main() {
    let args = Args::parse();

    if args.show_sizes {
        print_dirs_and_size();
    } else {
        print_dirs();
    }
}
