use std::env;

use anvil_region::AnvilRegion;
use nbt::CompoundTag;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Cursor, Read};

fn print_if_chunk_has_entity(chunk_nbt: &CompoundTag) -> Result<(), nbt::CompoundTagError> {
    let level = chunk_nbt.get_compound_tag("Level")?;

    let chunk_x = level.get_i32("xPos")?;
    let chunk_z = level.get_i32("zPos")?;

    let chunk_contains_entity = !level.get_compound_tag_vec("Entities")?.is_empty();
    let chunk_contains_tile_entity = !level.get_compound_tag_vec("TileEntities")?.is_empty();

    if chunk_contains_entity || chunk_contains_tile_entity {
        println!("({}, {})", chunk_x, chunk_z);
    }

    Ok(())
}

fn get_anvil_region_instance(region_file_path: &Path) -> std::io::Result<AnvilRegion<Cursor<Vec<u8>>>> {
    let file_contents = {
        let mut region_file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(region_file_path)?;
        let mut contents = Vec::new();
        region_file.read_to_end(&mut contents)?;
        contents
    };

    let region = AnvilRegion::new(Cursor::new(file_contents))?;
    Ok(region)
}

fn list_chunks_with_entities(region_path: &Path) -> () {
    for region_entry in region_path.read_dir().unwrap() {
        let region_file = region_entry.unwrap().path();
        let mut region = get_anvil_region_instance(&region_file).unwrap();

        for res in region.read_all_chunks().unwrap() {
            print_if_chunk_has_entity(&res).unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let world_folder_path: &Path = Path::new(&args[1]);
    let region_folder_path = world_folder_path.join("region");

    list_chunks_with_entities(&region_folder_path)
}
