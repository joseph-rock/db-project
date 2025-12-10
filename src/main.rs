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
    let _ = init_tables(&conn);

    let milk = Ingredient {
        id: 0,
        name: "Milk".to_string(),
        amount: 1.0,
        amount_type: "Gallon".to_string(),
    };
    let _ = add_ingredient(&conn, &milk);

    let cereal = Ingredient {
        id: 1,
        name: "Wheaties".to_string(),
        amount: 15.6,
        amount_type: "Ounce".to_string(),
    };
    let _ = add_ingredient(&conn, &cereal);

    let ingredients = select_all_ingredients(&conn).expect("trust me bro");
    dbg!(ingredients);

    let less_milk = Ingredient {
        id: 0,
        name: "Milk".to_string(),
        amount: 0.8,
        amount_type: "Gallon".to_string(),
    };

    let _ = update_ingredient(&conn, &less_milk);
    let ingredients = select_all_ingredients(&conn).expect("trust me bro");
    dbg!(ingredients);

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
            amount_type TEXT NOT NULL
        )",
        (),
    )
}

fn add_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO ingredients (name, amount, amount_type) 
            VALUES (?1, ?2, ?3);",
        (
            &ingredient.name,
            &ingredient.amount,
            &ingredient.amount_type,
        ),
    )
}

fn update_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE ingredients
            SET amount = ?1
            WHERE name = ?2;",
        (&ingredient.amount, &ingredient.name),
    )
}

fn select_ingredient(conn: &Connection, name: &str) -> Result<Ingredient> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amount, amount_type 
            FROM ingredients 
            WHERE name = ?1;",
    )?;
    stmt.query_one([name], |row| {
        Ok(Ingredient {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get(2)?,
            amount_type: row.get(3)?,
        })
    })
}

fn select_all_ingredients(conn: &Connection) -> Result<Vec<Ingredient>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amount, amount_type 
            FROM ingredients"
    )?;
    stmt.query_map([], |row| {
        Ok(Ingredient {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get(2)?,
            amount_type: row.get(3)?,
        })
    })?
    .collect()
}
