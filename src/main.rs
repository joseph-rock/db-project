mod measurements;

use measurements::*;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Ingredient {
    id: i32,
    name: String,
    measurement: Measurement,
}

fn main() -> Result<()> {
    // Init connection and create tables
    let conn = Connection::open_in_memory()?;
    let _ = init_tables(&conn);

    // Add milk & cereal
    let milk = Ingredient {
        id: 0,
        name: "Milk".to_string(),
        measurement: Measurement { 
            name: MeasurementName::Gallon, 
            amount: 1.0,
        },
    };
    let _ = add_ingredient(&conn, &milk);

    let cereal = Ingredient {
        id: 1,
        name: "Wheaties".to_string(),
        measurement: Measurement { 
            name: MeasurementName::Ounce, 
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
        measurement: Measurement { 
            name: MeasurementName::Gallon, 
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

fn init_tables(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE ingredients (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            amount FLOAT NOT NULL,
            measurement TEXT NOT NULL
        )",
        (),
    )
}

fn add_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO ingredients (name, amount, measurement) 
            VALUES (?1, ?2, ?3);",
        (&ingredient.name, &ingredient.measurement.amount, &ingredient.measurement.name.to_string()),
    )
}

fn update_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE ingredients
            SET amount = ?1
            WHERE name = ?2;",
        (&ingredient.measurement.amount, &ingredient.name),
    )
}

fn select_ingredient(conn: &Connection, name: &str) -> Result<Ingredient> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amount, measurement 
            FROM ingredients 
            WHERE name = ?1;",
    )?;
    stmt.query_one([name], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        let amount = row.get::<usize, f32>(2)?;
        let measurement_str = row.get::<usize, String>(3)?;
        Ok(Ingredient {
            id,
            name,
            measurement: Measurement {
                name: MeasurementName::from_string(&measurement_str).unwrap(),
                amount,
            },
        })
    })
}

fn select_all_ingredients(conn: &Connection) -> Result<Vec<Ingredient>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amount, measurement 
            FROM ingredients",
    )?;
    stmt.query_map([], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        let amount = row.get::<usize, f32>(2)?;
        let measurement_str = row.get::<usize, String>(3)?;
        Ok(Ingredient {
            id,
            name,
            measurement: Measurement {
                name: MeasurementName::from_string(&measurement_str).unwrap(),
                amount,
            },
        })
    })?
    .collect()
}

fn use_ingredient(conn: &Connection, name: &str, amount: f32) -> Result<(), rusqlite::Error> {
    let mut ingredient = select_ingredient(&conn, &name).unwrap();
    ingredient.measurement.amount = ingredient.measurement.amount - amount;
    let _ = update_ingredient(&conn, &ingredient);
    Ok(())
}
