// TODO brs-edit
//      ! Get import working for brs crate
//      ! Create a basic interpreter and REPL interface for editing
//      ? command-line arguments interface as well
// TODO REPL
//      ! change name/descriptions
//      % display info about save (# bricks, color count, ?mods)
//      ! basic operations
//      ! color -> another color 
//      ? brick -> another brick? if possible but doubtful
//      % randomization
//      ! write directly to file/save as new
//      ? load multiple files into buffer to edit all at once
//      ? delete bricks
// TODO release
//      ! Binaries for windows/linux
//      ! Get tests running
//      ! Make examples of REPL commands
//      ? CI/CD

use std::fs::File;
use brs::{Brick, Color, HasHeader1, HasHeader2};
use chrono::Utc;

fn main() -> Result<(), std::io::Error> {
    let reader = brs::Reader::new(File::open("Mini_Jungle.brs")?)?;
    let reader = reader.read_header1()?;
    let user = reader.author();
    let reader = reader.read_header2()?;
    let (reader, bricks) = reader.iter_bricks_and_reader()?;

    let bricks_to_save: Vec<Brick> = Vec::with_capacity(bricks.collect().length());

    for brick in bricks {
        brick = brick?;
        bricks.push(brick):
    }

    let mut colors = reader.colors();
    colors[2] = Color::from_rgba(255, 0, 255, 255);
    colors[3] = Color::from_rgba(255, 0, 255, 255);

    let data = brs::WriteData {
        author: *user,
        map: String::from("Plate"),
        description: String::from("test"),
        save_time: Utc::now(),
        mods: reader.mods().to_vec(),
        brick_assets: reader.brick_assets().to_vec(),
        colors: colors.to_vec(),
        materials: reader.materials().to_vec(),
        brick_owners: reader.brick_owners().to_vec(),
        bricks: bricks_to_save,
    };

    Ok(())
}

// fn write_new_save(
