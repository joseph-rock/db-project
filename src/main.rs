mod unit;

use rusqlite::{Connection, Error, Result, params};
use unit::*;

#[derive(Debug)]
struct Ingredient {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct Inventory {
    ingredient: Ingredient,
    unit: Unit,
}

// struct Recipe {
//     id: i32,
//     name: String,
//     ingredients: Vec<Ingredient>,
// }

fn main() -> Result<()> {
    // Init connection and create tables
    let conn = Connection::open_in_memory()?;
    let _ = init_tables(&conn);

    // Add milk & cereal
    let milka = Ingredient {
        id: 0,
        name: "Milk".to_string(),
    };
    let milkb = Inventory {
        ingredient: Ingredient {
            id: 0,
            name: "Milk".to_string(),
        },
        unit: Unit {
            name: UnitName::Gallon,
            amount: 15.0,
        },
    };
    let foo = add_ingredient(&conn, &milka);
    dbg!(foo);
    let bar = add_inventory(&conn, &milkb);
    dbg!(bar);
    let bazz = get_ingredient(&conn, &milka);
    dbg!(bazz);

    let cereal = Inventory {
        ingredient: Ingredient {
            id: 1,
            name: "Wheaties".to_string(),
        },
        unit: Unit {
            name: UnitName::Ounce,
            amount: 15.6,
        },
    };
    let _ = add_inventory(&conn, &cereal);
    let maybe_cereal = get_one_inventory(&conn, &cereal.ingredient.name);
    dbg!(maybe_cereal);

    let ingredients = get_all_inventory(&conn).expect("broke selecting all ingredients");
    dbg!(ingredients);

    Ok(())
}

fn init_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE ingredient(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE);

        CREATE TABLE inventory(
            id INTEGER PRIMARY KEY,
            amount INTEGER NOT NULL,
            amount_unit TEXT NOT NULL,
            FOREIGN KEY (id) REFERENCES ingredient(id));

        CREATE TABLE recipe(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL);

        CREATE TABLE recipe_ingredient(
            ingredient_id INTEGER NOT NULL,
            recipe_id INTEGER NOT NULL,
            amount INTEGER NOT NULL,
            amount_unit TEXT NOT NULL,
            FOREIGN KEY (ingredient_id) REFERENCES ingredient(id),
            FOREIGN KEY (recipe_id) REFERENCES recipe(id),
            PRIMARY KEY (ingredient_id, recipe_id));
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

fn add_inventory(conn: &Connection, inventory: &Inventory) -> Result<usize, Error> {
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

fn get_all_inventory(conn: &Connection) -> Result<Vec<Inventory>, Error> {
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
        Ok(Inventory {
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
fn get_one_inventory(conn: &Connection, name: &str) -> Result<Inventory, Error> {
    let mut stmt = conn.prepare(
        "SELECT ingredient.id, ingredient.name, inventory.amount, inventory.amount_unit
            FROM inventory
            JOIN ingredient ON ingredient.id = inventory.id
            WHERE name = ?1",
    )?;
    stmt.query_one([name], |row| {
        let id = row.get(0)?;
        let ingredient_name = row.get(1)?;
        let amount = row.get::<usize, f64>(2)?;
        let unit_str = row.get::<usize, String>(3)?;
        let unit_name = UnitName::from_string(&unit_str).unwrap();
        Ok(Inventory {
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

fn update_inventory(conn: &Connection, inventory: &Inventory) -> Result<usize, Error> {
    conn.execute(
        "UPDATE inventory
            SET amount = ?1
            WHERE name = ?2;",
        (&inventory.unit.amount, &inventory.ingredient.name),
    )
}

// fn use_ingredient(conn: &Connection, name: &str, amount: f64) -> Result<usize, Error> {
//     let mut ingredient = select_ingredient(&conn, &name).unwrap();
//     ingredient.unit.amount = ingredient.unit.amount - amount;
//     update_ingredient(&conn, &ingredient)
// }
