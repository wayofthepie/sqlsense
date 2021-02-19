mod dotify;
mod sql;
use sql::map_relationships;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() < 2 {
        return Err("You must pass two args, the path to a sql file, and the path to the dot file to write, thanks!".into());
    }
    let path = args.get(0).unwrap();
    let out = args.get(1).unwrap();
    let sql = std::fs::read_to_string(&path)?;
    let relationships = map_relationships(&sql);
    let mut f = File::create(out).unwrap();
    dotify::render_to(&relationships, &mut f);
    Ok(())
}
