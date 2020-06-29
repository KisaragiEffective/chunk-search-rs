use std::env;

use anvil_region::AnvilRegion;
use nbt::CompoundTag;
use std::path::Path;

fn print_if_chunk_has_entity(chunk_nbt: &CompoundTag) -> Result<(), nbt::CompoundTagError> {
    let chunk_x = chunk_nbt.get_i32("xPos")?;
    let chunk_z = chunk_nbt.get_i32("zPos")?;

    let chunk_contains_entity = !chunk_nbt.get_compound_tag_vec("Entities")?.is_empty();
    let chunk_contains_tile_entity = !chunk_nbt.get_compound_tag_vec("TileEntities")?.is_empty();

    if chunk_contains_entity || chunk_contains_tile_entity {
        println!("({}, {})", chunk_x, chunk_z);
    }

    Ok(())
}

fn list_chunks_with_entities(region_path: &Path) -> Option<()> {
    for region_file in region_path.read_dir().ok()? {
        let mut region: AnvilRegion = AnvilRegion::from_path(region_file.ok()?.path()).ok()?;

        for chunk in region.read_all_chunks().ok()? {
            print_if_chunk_has_entity(&chunk).ok()?
        }
    }

    Some(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let world_folder_path: &Path = Path::new(&args[0]);

    list_chunks_with_entities(&world_folder_path.join("region")).unwrap()
}
