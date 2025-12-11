mod unit;

use unit::*;
use rusqlite::{Connection, Result, Error};

#[derive(Debug)]
struct Ingredient {
    id: i32,
    name: String,
    unit: Unit,
}

fn main() -> Result<()> {
    // Init connection and create tables
    let conn = Connection::open_in_memory()?;
    let _ = init_tables(&conn);

    // Add milk & cereal
    let milk = Ingredient {
        id: 0,
        name: "Milk".to_string(),
        unit: Unit { 
            name: UnitName::Gallon, 
            amount: 1.0,
        },
    };
    let _ = add_ingredient(&conn, &milk);

    let cereal = Ingredient {
        id: 1,
        name: "Wheaties".to_string(),
        unit: Unit { 
            name: UnitName::Ounce, 
            amount: 15.6,
        },
    };
    let _ = add_ingredient(&conn, &cereal);

    let ingredients = select_all_ingredients(&conn).expect("trust me bro");
    dbg!(ingredients);

    // Update milk amount
    let less_milk = Ingredient {
        id: 0,
        name: "Milk".to_string(),
        unit: Unit { 
            name: UnitName::Gallon, 
            amount: 1.5,
        },
    };

    let _ = update_ingredient(&conn, &less_milk);
    let ingredients = select_all_ingredients(&conn).expect("trust me bro");
    dbg!(ingredients);

    // Get single ingredient
    let get_cereal = select_ingredient(&conn, "Wheaties").expect("trust me bro");
    dbg!(&get_cereal);

    // Use an amount of an ingredient
    let _ = use_ingredient(&conn, "Wheaties", 2.0);
    let get_cereal = select_ingredient(&conn, "Wheaties").expect("trust me bro");
    dbg!(&get_cereal);

    Ok(())
}

fn init_tables(conn: &Connection) -> Result<usize, Error> {
    conn.execute(
        "CREATE TABLE ingredients (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            amount FLOAT NOT NULL,
            unit TEXT NOT NULL
        )",
        (),
    )
}

fn add_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, Error> {
    conn.execute(
        "INSERT INTO ingredients (name, amount, unit) 
            VALUES (?1, ?2, ?3);",
        (&ingredient.name, &ingredient.unit.amount, &ingredient.unit.name.to_string()),
    )
}

fn update_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, Error> {
    conn.execute(
        "UPDATE ingredients
            SET amount = ?1
            WHERE name = ?2;",
        (&ingredient.unit.amount, &ingredient.name),
    )
}

fn select_ingredient(conn: &Connection, name: &str) -> Result<Ingredient, Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amount, unit 
            FROM ingredients 
            WHERE name = ?1;",
    )?;
    stmt.query_one([name], |row| {
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
    })
}

fn select_all_ingredients(conn: &Connection) -> Result<Vec<Ingredient>, Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amount, unit 
            FROM ingredients",
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

fn use_ingredient(conn: &Connection, name: &str, amount: f64) -> Result<usize, Error> {
    let mut ingredient = select_ingredient(&conn, &name).unwrap();
    ingredient.unit.amount = ingredient.unit.amount - amount;
    update_ingredient(&conn, &ingredient)
}
