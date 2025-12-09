use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Ingredient {
    id: i32,
    name: String,
    amount: f32,
    amount_type: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE ingredients (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            amount  FLOAT NOT NULL,
            amount_type  TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    let milk = Ingredient {
        id: 0,
        name: "Milk".to_string(),
        amount: 1.0,
        amount_type: "Gallon".to_string(),
    };

    let cereal = Ingredient {
        id: 1,
        name: "Wheaties".to_string(),
        amount: 15.6,
        amount_type: "Ounce".to_string(),
    };

    let _ = insert(&conn, &milk);
    let _ = insert(&conn, &cereal);

    let mut stmt = conn.prepare("SELECT id, name, amount, amount_type FROM ingredients")?;
    let ingredients = stmt.query_map([], |row| {
        Ok(Ingredient {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get(2)?,
            amount_type: row.get(3)?,
        })
    })?;

    for ingredient in ingredients {
        println!("{:?}", ingredient.unwrap());
    }
    Ok(())
}

fn insert(conn: &Connection, ingredient: &Ingredient) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO ingredients (name, amount, amount_type) VALUES (?1, ?2, ?3)",
        (&ingredient.name, &ingredient.amount, &ingredient.amount_type),
    )
}
