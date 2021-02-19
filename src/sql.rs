use sqlparser::{
    ast::{ColumnDef, ColumnOption::ForeignKey, Ident, ObjectName, Statement::CreateTable},
    dialect::GenericDialect,
    parser::Parser,
};
use std::collections::HashMap;

pub fn map_relationships(sql: &str) -> HashMap<String, Vec<String>> {
    let dialect = GenericDialect;
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
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

fn get_foreign_keys_for_columns(columns: &[ColumnDef]) -> Vec<String> {
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
                        identifiers.get(0).map(|id| id.value.clone())
                    }
                    _ => None,
                })
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    use crate::map_relationships;
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
        expected.insert("events".to_owned(), vec!["event_types".to_owned()]);
        expected.insert("team".to_owned(), vec![]);
        expected.insert("dev".to_owned(), vec!["team".to_owned()]);
        expected.insert("event_types".to_owned(), vec![]);

        let keys = map_relationships(sql);
        assert_eq!(keys, expected);
    }
}
