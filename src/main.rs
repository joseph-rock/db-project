use rusqlite::{Connection, Error, Result, params};

#[derive(Debug)]
struct RecipeIngredient {
    ingredient: String,
    amount: usize,
    unit: String,
}

struct Ingredient {
    id: usize,
    name: String,
}

fn main() -> Result<(), Error> {
    let conn = Connection::open_in_memory()?;

    // TODO: Error handling
    let _ = init_tables(&conn);


    Ok(())
}

fn init_tables(conn: &Connection) -> Result<(), Error> {
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
          ingredient integer not null references ingredients,
          amount integer not null,
          unit integer not null references units
        ) strict;

        create table if not exists recipe_ingredients (
          ingredient integer not null references ingredients,
          recipe integer not null references recipes,
          amount integer not null,
          unit integer not null references units,
          PRIMARY KEY (ingredient, recipe)
        ) strict;
        ",
    )?;

    Ok(())
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

fn ingredient_exists(conn: &Connection, name: &str) -> Result<bool, Error> {
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
// fn select_unit(conn: &Connection, name: &str) -> Result<Unit, Error> {
//     let mut stmt = conn.prepare("select id, name from units where name = ?1;")?;
//     stmt.query_one(params![name], |row| {
//         let id = row.get(0)?;
//         let name = row.get(1)?;
//         Ok(Unit { id, name })
//     })
// }

fn unit_exists(conn: &Connection, name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("select * from units where name = ?1;")?;
    stmt.exists(params![name])
}

fn insert_inventory(
    conn: &Connection,
    ingredient_name: &str,
    amount: usize,
    unit_name: &str,
) -> Result<usize, Error> {
    // TODO: all units should be populated, error if unit does not exist
    if !unit_exists(&conn, &unit_name)? {
        insert_unit(&conn, &unit_name)?;
    }

    if !ingredient_exists(&conn, &ingredient_name)? {
        insert_ingredient(&conn, &ingredient_name)?;
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
    stmt.execute(params![ingredient_name, amount, unit_name])
}

fn insert_recipe_name(conn: &Connection, name: &str) -> Result<usize, Error> {
    let mut stmt = conn.prepare("insert into recipes (name) values (?1);")?;
    stmt.execute(params![name])
}

fn recipe_name_exists(conn: &Connection, name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("select * from recipes where name = ?1;")?;
    stmt.exists(params![name])
}

fn insert_recipe_ingredient(
    conn: &Connection,
    recipe_name: &str,
    ingredient_name: &str,
    unit_name: &str,
    amount: &usize,
) -> Result<usize, Error> {
    if !recipe_name_exists(&conn, &recipe_name)? {
        insert_recipe_name(&conn, &recipe_name)?;
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
            (select id from recipes where name = ?2),
            (?3),
            (select id from units where name = ?4)
          );
        ",
    )?;

    stmt.execute(params![ingredient_name, recipe_name, amount, unit_name])
}

fn insert_recipe(
    conn: &Connection,
    recipe_name: &str,
    ingredients: &Vec<RecipeIngredient>,
) -> Result<(), Error> {
    for ingredient in ingredients {
        insert_recipe_ingredient(
            &conn,
            &recipe_name,
            &ingredient.ingredient,
            &ingredient.unit,
            &ingredient.amount,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingredient() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init_tables(&conn)?;

        let ingredient_name = "Milk".to_string();
        insert_ingredient(&conn, &ingredient_name)?;

        let select_ingredient = select_ingredient(&conn, &ingredient_name)?;
        assert_eq!(select_ingredient.name, ingredient_name);

        Ok(())
    }

    #[test]
    fn recipe() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init_tables(&conn)?;

        let recipe_name = "Bowl of Cereal".to_string();
        let milk = RecipeIngredient {
            ingredient: "Milk".to_string(),
            amount: 1,
            unit: "Cup".to_string(),
        };
        let wheaties = RecipeIngredient {
            ingredient: "Wheaties".to_string(),
            amount: 1,
            unit: "Cup".to_string(),
        };

        let ingredients = vec![milk, wheaties];

        insert_recipe(&conn, &recipe_name, &ingredients)?;

        Ok(())
    }
}
