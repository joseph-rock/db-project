mod unit;

use rusqlite::{Connection, Error, Result, params, ErrorCode};
use unit::*;

#[derive(Debug)]
struct Ingredient {
    id: i32,
    name: String,
    unit: Unit,
}

struct Recipie {
    id: i32,
    name: String,
    ingredients: Vec<Ingredient>,
}

fn main() -> Result<()> {
    // Init connection and create tables
    let conn = Connection::open_in_memory()?;
    let _ = init_tables(&conn);

    // Add milk & cereal
    let milka = Ingredient {
        id: 0,
        name: "Milk".to_string(),
        unit: Unit {
            name: UnitName::Gallon,
            amount: 1.0,
        },
    };
    let milkb = Ingredient {
        id: 0,
        name: "Milk".to_string(),
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
    //
    // let cereal = Ingredient {
    //     id: 1,
    //     name: "Wheaties".to_string(),
    //     unit: Unit {
    //         name: UnitName::Ounce,
    //         amount: 15.6,
    //     },
    // };
    // let _ = add_ingredient(&conn, &milk);
    // let _ = add_ingredient(&conn, &cereal);
    //
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

// TODO: return some struct, not tuple
fn get_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<(usize, String), Error> {
    let mut stmt = conn.prepare("SELECT id, name FROM ingredient WHERE name = ?1")?;
    stmt.query_one([&ingredient.name], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        Ok((id, name))
    })
}

fn add_inventory(conn: &Connection, ingredient: &Ingredient) -> Result<usize, Error> {
    // try add ingredient first
    // *could* throw away return, expect to work or return "ConstraintViolation" which is fine
    // TODO: should handle other error
    let _ = add_ingredient(&conn, &ingredient);

    let (id, _) = get_ingredient(&conn, &ingredient)?;
    let amount = &ingredient.unit.amount;
    let amount_unit = &ingredient.unit.name.to_string();

    let mut inventory_stmt = conn.prepare("INSERT INTO inventory(id, amount, amount_unit) VALUES (?1, ?2, ?3);")?;
    inventory_stmt.execute(params![id, amount, amount_unit])
}

fn get_all_inventory(conn: &Connection) -> Result<Vec<Ingredient>, Error> {
    let mut stmt = conn.prepare(
        "SELECT ingredient.id, ingredient.name, inventory.amount, inventory.amount_unit
            FROM inventory
            JOIN ingredient ON ingredient.id = inventory.id;"
    )?;
    stmt.query_map([], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        let amount = row.get::<usize, f64>(2)?;
        let unit_str = row.get::<usize, String>(3)?;
        Ok(Ingredient {
            id,
            name,
            unit: Unit {
                name: UnitName::from_string(&unit_str).unwrap(),
                amount,
            },
        })
    })?
    .collect()
}

// fn update_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, Error> {
//     conn.execute(
//         "UPDATE inventory
//             SET amount = ?1
//             WHERE name = ?2;",
//         (&ingredient.unit.amount, &ingredient.name),
//     )
// }
//
// fn select_ingredient(conn: &Connection, name: &str) -> Result<Ingredient, Error> {
//     let mut stmt = conn.prepare(
//         "SELECT id, name, amount, amount_unit
//             FROM inventory
//             WHERE name = ?1",
//     )?;
//     stmt.query_one([name], |row| {
//         let id = row.get(0)?;
//         let name = row.get(1)?;
//         let amount = row.get::<usize, f64>(2)?;
//         let unit_str = row.get::<usize, String>(3)?;
//         Ok(Ingredient {
//             id,
//             name,
//             unit: Unit {
//                 name: UnitName::from_string(&unit_str).unwrap(),
//                 amount,
//             },
//         })
//     })
// }


// fn use_ingredient(conn: &Connection, name: &str, amount: f64) -> Result<usize, Error> {
//     let mut ingredient = select_ingredient(&conn, &name).unwrap();
//     ingredient.unit.amount = ingredient.unit.amount - amount;
//     update_ingredient(&conn, &ingredient)
// }
