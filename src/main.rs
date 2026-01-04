mod unit;

use rusqlite::{Connection, Error, Result, params};
use unit::*;

#[derive(Debug, Clone)]
struct Ingredient {
    id: i32,
    name: String,
}

// #[derive(Debug)]
// struct Inventory {
//     ingredient: Ingredient,
//     unit: Unit,
// }
//
// struct Recipe {
//     id: i32,
//     name: String,
//     ingredients: Vec<Inventory>,
// }

fn main() -> Result<()> {
    // Init connection and create tables
    let conn = Connection::open_in_memory()?;
    let _ = init_tables(&conn);

    Ok(())
}

fn init_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        create table if not exists ingredients (
          id integer primary key,
          name text not null
        ) strict;

        create table if not exists units (
          id integer primary key,
          name text not null
        ) strict;

        create table if not exists recipes (
          id integer primary key,
          name text not null,
          description text not null
        ) strict;

        create table if not exists inventory (
          id integer primary key,
          amount integer not null,
          unit integer not null references units,
          ingredient integer not null references ingredients
        ) strict;

        create table if not exists recipe_ingredients (
          ingredient integer not null references ingredients,
          recipe integer not null references recipes,
          amount integer not null,
          unit integer not null references units,
          PRIMARY KEY (ingredient, recipe)
        ) strict;
        ",
    )
}

fn insert_ingredient(conn: &Connection, name: &str) -> Result<usize, Error> {
    let mut stmt = conn.prepare("insert into ingredients (name) values (?1);")?;
    stmt.execute(params![name])
}

// query_one
//  returns Err(QueryReturnedMoreThanOneRow)
//  returns Err(QueryReturnedNoRows)
fn select_ingredient(conn: &Connection, name: &str) -> Result<Ingredient, Error> {
    let mut stmt = conn.prepare("select id, name from ingredients where name = ?1;")?;
    stmt.query_one(params![name], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        Ok(Ingredient { id, name })
    })
}

// This table should be pre-populated -- no one is entering units
fn insert_unit(conn: &Connection, name: &str) -> Result<usize, Error> {
    let mut stmt = conn.prepare("insert into units (name) values (?1);")?;
    stmt.execute(params![name])
}

fn insert_inventory(conn: &Connection, inventory: (usize, String, String)) -> Result<usize, Error> {
    // if unit does not exist, insert_unit()

    // if ingredient does not exist, insert_ingredient()
    let mut stmt = conn.prepare(
        "
        insert into inventory (amount, unit, ingredient)
        values (
            ?1, 
            (select id from units where name = ?2), 
            (select id from ingredients where name = ?3)
          );
        ",
    )?;
    stmt.execute(params![inventory.0, inventory.1, inventory.2])
}

