use rusqlite::{Connection, Error, Result, params};

mod unit;
use unit::{Unit, UnitName};

#[derive(Debug)]
struct RecipeIngredient {
    ingredient: String,
    unit: Unit,
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
          name text unique not null
        ) strict;

        create table if not exists units (
          id integer primary key,
          name text unique not null
        ) strict;

        create table if not exists recipes (
          id integer primary key,
          name text unique not null
        ) strict;

        create table if not exists inventory (
          id integer primary key,
          ingredient integer not null references ingredients,
          amount real not null,
          unit integer not null references units
        ) strict;

        create table if not exists recipe_ingredients (
          ingredient integer not null references ingredients,
          recipe integer not null references recipes,
          amount integer not null,
          unit real not null references units,
          PRIMARY KEY (ingredient, recipe)
        ) strict;
        ",
    )?;

    Ok(())
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase()
}

fn insert_ingredient(conn: &Connection, name: &str) -> Result<usize, Error> {
    let name = normalize_name(name);
    let mut stmt = conn.prepare("insert or ignore into ingredients (name) values (?1);")?;
    stmt.execute(params![name])
}

// query_one
//      returns Err(QueryReturnedMoreThanOneRow)
//      returns Err(QueryReturnedNoRows)
fn select_ingredient(conn: &Connection, name: &str) -> Result<Ingredient, Error> {
    let name = normalize_name(name);
    let mut stmt = conn.prepare("select id, name from ingredients where name = ?1;")?;
    stmt.query_one(params![name], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        Ok(Ingredient { id, name })
    })
}

// This table should be pre-populated -- no one is entering units
fn insert_unit(conn: &Connection, name: &str) -> Result<usize, Error> {
    let name = normalize_name(name);
    let mut stmt = conn.prepare("insert or ignore into units (name) values (?1);")?;
    stmt.execute(params![name])
}

fn insert_inventory(conn: &Connection, ingredient_name: &str, unit: &Unit) -> Result<usize, Error> {
    let unit_name = normalize_name(&unit.name.to_string());
    let ingredient_name = normalize_name(ingredient_name);

    insert_unit(&conn, &unit_name)?;
    insert_ingredient(&conn, &ingredient_name)?;

    let mut stmt = conn.prepare(
        "
        insert into inventory (ingredient, amount, unit)
        values (
            (select id from ingredients where name = ?1),
            (?2),
            (select id from units where name = ?3)
          );
        ",
    )?;
    stmt.execute(params![ingredient_name, unit.amount, unit_name])
}

fn insert_recipe_name(conn: &Connection, name: &str) -> Result<usize, Error> {
    let name = normalize_name(name);
    let mut stmt = conn.prepare("insert or ignore into recipes (name) values (?1);")?;
    stmt.execute(params![name])
}

fn insert_recipe_ingredient(
    conn: &Connection,
    recipe_name: &str,
    ingredient_name: &str,
    unit: &Unit,
) -> Result<usize, Error> {
    let recipe_name = normalize_name(recipe_name);
    let ingredient_name = normalize_name(ingredient_name);
    let unit_name = normalize_name(&unit.name.to_string());

    insert_recipe_name(&conn, &recipe_name)?;
    insert_ingredient(&conn, &ingredient_name)?;
    insert_unit(&conn, &unit_name)?;

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

    stmt.execute(params![
        ingredient_name,
        recipe_name,
        unit.amount,
        unit_name,
    ])
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

        Ok(())
    }

    #[test]
    fn inventory() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init_tables(&conn)?;

        let ingredient_name = "Milk".to_string();
        let unit = Unit {
            amount: 1.0,
            name: UnitName::Gallon,
        };

        insert_inventory(&conn, &ingredient_name, &unit)?;

        Ok(())
    }

    #[test]
    fn recipe() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init_tables(&conn)?;

        let recipe_name = "Bowl of Cereal".to_string();
        let milk = RecipeIngredient {
            ingredient: "Milk".to_string(),
            unit: Unit {
                amount: 1.0,
                name: UnitName::Cup,
            },
        };
        let wheaties = RecipeIngredient {
            ingredient: "Wheaties".to_string(),
            unit: Unit {
                amount: 1.0,
                name: UnitName::Cup,
            },
        };

        let ingredients = vec![milk, wheaties];

        insert_recipe(&conn, &recipe_name, &ingredients)?;

        Ok(())
    }
}
