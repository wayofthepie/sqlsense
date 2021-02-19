mod dotify;
use sqlparser::{
    ast::{
        ColumnDef,
        ColumnOption::ForeignKey,
        Ident, ObjectName,
        Statement::{self, CreateTable},
    },
    dialect::GenericDialect,
    parser::Parser,
};
use std::fs::File;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() < 2 {
        return Err("You must pass two args, the path to a sql file, and the path to the dot file to write, thanks!".into());
    }
    let path = args.get(0).unwrap();
    let out = args.get(1).unwrap();
    let sql = std::fs::read_to_string(&path)?;
    let dialect = GenericDialect;
    let ast = Parser::parse_sql(&dialect, &sql).unwrap();
    let relationships = map_relationships(&ast);
    let mut f = File::create(out).unwrap();
    dotify::render_to(&relationships, &mut f);
    Ok(())
}

fn map_relationships(ast: &[Statement]) -> HashMap<String, Vec<&str>> {
    let mut relationships = HashMap::new();
    ast.iter().for_each(|statement| {
        if let CreateTable {
            name: ObjectName(identifiers),
            columns,
            ..
        } = statement
        {
            // TODO read up on idents, the extraction here may be incorrect in general
            if let Some(&Ident { ref value, .. }) = identifiers.get(0) {
                let fks = get_foreign_keys_for_columns(&columns);
                relationships.insert(value.to_owned(), fks);
            }
        }
    });
    relationships
}

fn get_foreign_keys_for_columns(columns: &[ColumnDef]) -> Vec<&str> {
    columns
        .iter()
        .flat_map(|column| {
            column
                .options
                .iter()
                .filter_map(|option_def| match &option_def.option {
                    ForeignKey {
                        foreign_table: ObjectName(identifiers),
                        ..
                    } => {
                        // TODO read up on idents, the extraction here may be incorrect in general
                        identifiers.get(0).map(|id| id.value.as_ref())
                    }
                    _ => None,
                })
        })
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod test {
    use crate::map_relationships;
    use sqlparser::{dialect::GenericDialect, parser::Parser};
    use std::collections::HashMap;

    #[test]
    fn should_return_event_types_as_foreign_key() {
        let sql = r#"
        create table event_types(id serial primary key, event_type text);
        create table events (id serial primary key, event_type integer references event_types);
        create table team (id serial primary key);
        create table dev (id serial primary key, team_id integer references team);
        "#;

        let mut expected = HashMap::new();
        expected.insert("events".to_owned(), vec!["event_types"]);
        expected.insert("team".to_owned(), vec![]);
        expected.insert("dev".to_owned(), vec!["team"]);
        expected.insert("event_types".to_owned(), vec![]);

        let dialect = GenericDialect;
        let ast = Parser::parse_sql(&dialect, &sql).unwrap();

        let keys = map_relationships(&ast);
        assert_eq!(keys, expected);
    }
}
