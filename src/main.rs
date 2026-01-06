use rusqlite::{Connection, Error, Result, params};

#[derive(Debug, Clone)]
struct Ingredient {
    id: usize,
    name: String,
}

#[derive(Debug, Clone)]
struct Unit {
    id: usize,
    name: String,
}

#[derive(Debug)]
struct Inventory {
    id: usize,
    ingredient: Ingredient,
    amount: usize,
    unit: Unit,
}

#[derive(Debug)]
struct Recipe {
    id: usize,
    name: String,
    ingredients: Vec<RecipeIngredient>,
}

#[derive(Debug)]
struct RecipeIngredient {
    ingredient: Ingredient,
    amount: usize,
    unit: Unit,
}

fn main() -> Result<()> {
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
          name text not null
        ) strict;

        create table if not exists inventory (
          id integer primary key,
          ingredient integer not null references ingredients
          amount integer not null,
          unit integer not null references units,
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
//      returns Err(QueryReturnedMoreThanOneRow)
//      returns Err(QueryReturnedNoRows)
fn select_ingredient(conn: &Connection, name: &str) -> Result<Ingredient, Error> {
    let mut stmt = conn.prepare("select id, name from ingredients where name = ?1;")?;
    stmt.query_one(params![name], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        Ok(Ingredient { id, name })
    })
}

fn ingredient_exists(conn: &Connection, name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("select * from ingredients where name = ?1;")?;
    stmt.exists(params![name])
}

// This table should be pre-populated -- no one is entering units
fn insert_unit(conn: &Connection, name: &str) -> Result<usize, Error> {
    let mut stmt = conn.prepare("insert into units (name) values (?1);")?;
    stmt.execute(params![name])
}

// query_one
//      returns Err(QueryReturnedMoreThanOneRow)
//      returns Err(QueryReturnedNoRows)
fn select_unit(conn: &Connection, name: &str) -> Result<Unit, Error> {
    let mut stmt = conn.prepare("select id, name from units where name = ?1;")?;
    stmt.query_one(params![name], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        Ok(Unit { id, name })
    })
}

fn unit_exists(conn: &Connection, name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("select * from units where name = ?1;")?;
    stmt.exists(params![name])
}

fn insert_inventory(conn: &Connection, inventory: Inventory) -> Result<usize, Error> {
    // TODO: all units should be populated, error if unit does not exist
    if !unit_exists(&conn, &inventory.unit.name)? {
        insert_unit(&conn, &inventory.unit.name)?;
    }

    if !ingredient_exists(&conn, &inventory.ingredient.name)? {
        insert_ingredient(&conn, &inventory.ingredient.name)?;
    }

    let mut stmt = conn.prepare(
        "
        insert into inventory (ingredient, amount, unit)
        values (
            (select id from ingredients where name = ?1),
            ?2,
            (select id from units where name = ?3), 
          );
        ",
    )?;
    stmt.execute(params![
        inventory.ingredient.name,
        inventory.amount,
        inventory.unit.name
    ])
}

fn insert_recipe(conn: &Connection, name: &str) -> Result<usize, Error> {
    let mut stmt = conn.prepare("insert into recipes (name) values (?1);")?;
    stmt.execute(params![name])
}

fn recipe_exists(conn: &Connection, name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("select * from recipes where name = ?1;")?;
    stmt.exists(params![name])
}

fn insert_recipe_ingredient(
    conn: &Connection,
    recipe_name: &str,
    ingredient_name: &str,
    unit_name: &str,
    amount: usize,
) -> Result<usize, Error> {
    if !recipe_exists(&conn, &recipe_name)? {
        insert_recipe(&conn, &recipe_name)?;
    }
    if !ingredient_exists(&conn, &ingredient_name)? {
        insert_ingredient(&conn, &ingredient_name)?;
    }
    // TODO: all units should be populated, error if unit does not exist
    if !unit_exists(&conn, &unit_name)? {
        insert_unit(&conn, &unit_name)?;
    }

    let mut stmt = conn.prepare(
        "
        insert into recipe_ingredients (ingredient, recipe, amount, unit)
        values (
            (select id from ingredients where name = ?1),
            (select id from recipes where name = ?2)
            ?3,
            (select id from units where name = ?4), 
          );
        ",
    )?;

    stmt.execute(params![ingredient_name, recipe_name, amount, recipe_name])
}
