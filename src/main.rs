mod unit;

use rusqlite::{Connection, Error, Result, params};
use unit::*;

#[derive(Debug, Clone)]
struct Ingredient {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct IngredientUnit {
    ingredient: Ingredient,
    unit: Unit,
}

struct Recipe {
    id: i32,
    name: String,
    ingredients: Vec<IngredientUnit>,
}

fn main() -> Result<()> {
    // Init connection and create tables
    let conn = Connection::open_in_memory()?;
    let _ = init_tables(&conn);

    Ok(())
}

fn init_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
            create table ingredients (
              id integer primary key,
              name text not null unique
            ) strict;

            create table inventory (
              id integer primary key,
              amount integer not null,
              unit integer not null references units,
              ingredient integer not null refrences ingredients
            ) strict;

            create table recipes (
              id integer primary key,
              name text not null,
              description text not null
            ) strict;

            create table recipe_ingredients (
              ingredient integer not null references ingredients,
              recipe integer not null references recipes,
              amount integer not null,
              unit integer not null references units,
              PRIMARY KEY (ingredient, recipe)
            ) strict;

            create table units (
              id integer primary key,
              name text not null
            ) strict;
        ",
    )
}

fn add_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, Error> {
    let mut inventory_stmt = conn.prepare("INSERT INTO ingredient(name) VALUES (?1);")?;
    inventory_stmt.execute(params![&ingredient.name])
}

fn get_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<Ingredient, Error> {
    let mut stmt = conn.prepare("SELECT id, name FROM ingredient WHERE name = ?1")?;
    stmt.query_one([&ingredient.name], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        Ok(Ingredient { id, name })
    })
}

fn add_inventory(conn: &Connection, inventory: &IngredientUnit) -> Result<usize, Error> {
    // try add ingredient first
    // *could* throw away return, expect to work or return "ConstraintViolation" which is fine
    // TODO: should handle other errors
    let _ = add_ingredient(&conn, &inventory.ingredient);
    let ingredient = get_ingredient(&conn, &inventory.ingredient)?;
    let amount = &inventory.unit.amount;
    let amount_unit = &inventory.unit.name.to_string();

    let mut inventory_stmt = conn.prepare(
        "
        INSERT INTO inventory(id, amount, amount_unit) 
        VALUES (?1, ?2, ?3);
        ",
    )?;
    inventory_stmt.execute(params![ingredient.id, amount, amount_unit])
}

fn get_all_inventory(conn: &Connection) -> Result<Vec<IngredientUnit>, Error> {
    let mut stmt = conn.prepare(
        "SELECT ingredient.id, ingredient.name, inventory.amount, inventory.amount_unit
            FROM inventory
            JOIN ingredient ON ingredient.id = inventory.id;",
    )?;
    stmt.query_map([], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        let amount = row.get::<usize, f64>(2)?;
        let unit_str = row.get::<usize, String>(3)?;
        Ok(IngredientUnit {
            ingredient: Ingredient { id, name },
            unit: Unit {
                name: UnitName::from_string(&unit_str).unwrap(),
                amount,
            },
        })
    })?
    .collect()
}

// TODO: this naming is awful
fn get_one_inventory(conn: &Connection, name: &str) -> Result<IngredientUnit, Error> {
    let mut stmt = conn.prepare(
        "SELECT ingredient.id, ingredient.name, inventory.amount, inventory.amount_unit
            FROM inventory
            JOIN ingredient ON ingredient.id = inventory.id
            WHERE name = ?1;",
    )?;
    stmt.query_one([name], |row| {
        let id = row.get(0)?;
        let ingredient_name = row.get(1)?;
        let amount = row.get::<usize, f64>(2)?;
        let unit_str = row.get::<usize, String>(3)?;
        let unit_name = UnitName::from_string(&unit_str).unwrap();
        Ok(IngredientUnit {
            ingredient: Ingredient {
                id,
                name: ingredient_name,
            },
            unit: Unit {
                name: unit_name,
                amount,
            },
        })
    })
}

fn update_inventory(conn: &Connection, inventory: &IngredientUnit) -> Result<usize, Error> {
    conn.execute(
        "UPDATE inventory
            SET amount = ?1
            WHERE name = ?2;",
        (&inventory.unit.amount, &inventory.ingredient.name),
    )
}

fn get_one_recipe(conn: &Connection, name: &str) -> Result<Recipe, Error> {
    let mut stmt = conn.prepare(
        "SELECT ingredient.name, recipe.name, recipe_ingredient.amount, recipe_ingredient.amount_unit
        FROM recipe_ingredient
        JOIN ingredient ON ingredient.id = recipe_ingredient.ingredient_id
        JOIN recipe ON recipe.id = recipe_ingredient.recipe_id
        WHERE recipe.name = ?1;"
        )?;

    todo!()
}
