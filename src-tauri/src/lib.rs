use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Serialize)]
struct Person {
    id: i64,
    last_name: String,
    first_name: String,
    middle_name: String,
    sex: String,
    birth_date: String,
    birth_place: String,
    father: String,
    mother: String,
    residence: String,
    occupation: String,
    legitimacy: String,
    midwife: String,
    godparents: String,
    priest: String,
    notes: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct NewPerson {
    last_name: String,
    first_name: String,
    middle_name: String,
    sex: String,
    birth_date: String,
    birth_place: String,
    father: String,
    mother: String,
    residence: String,
    occupation: String,
    legitimacy: String,
    midwife: String,
    godparents: String,
    priest: String,
    notes: String,
}

fn db_path(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&base).map_err(|e| e.to_string())?;
    Ok(base.join("dzherelo.sqlite3"))
}

fn open_db(app: &tauri::AppHandle) -> Result<Connection, String> {
    let path = db_path(app)?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS persons (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          last_name TEXT NOT NULL,
          first_name TEXT NOT NULL,
          middle_name TEXT NOT NULL,
          sex TEXT NOT NULL,
          birth_date TEXT NOT NULL,
          birth_place TEXT NOT NULL,
          father TEXT NOT NULL,
          mother TEXT NOT NULL,
          residence TEXT NOT NULL,
          occupation TEXT NOT NULL,
          legitimacy TEXT NOT NULL,
          midwife TEXT NOT NULL,
          godparents TEXT NOT NULL,
          priest TEXT NOT NULL,
          notes TEXT NOT NULL,
          created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        "#,
    )
    .map_err(|e| e.to_string())?;
    Ok(conn)
}

#[tauri::command]
fn list_people(app: tauri::AppHandle) -> Result<Vec<Person>, String> {
    let conn = open_db(&app)?;
    let mut stmt = conn
        .prepare(
            r#"
            SELECT id, last_name, first_name, middle_name, sex, birth_date,
                   birth_place, father, mother, residence, occupation,
                   legitimacy, midwife, godparents, priest, notes, created_at
            FROM persons
            ORDER BY id DESC
            "#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                last_name: row.get(1)?,
                first_name: row.get(2)?,
                middle_name: row.get(3)?,
                sex: row.get(4)?,
                birth_date: row.get(5)?,
                birth_place: row.get(6)?,
                father: row.get(7)?,
                mother: row.get(8)?,
                residence: row.get(9)?,
                occupation: row.get(10)?,
                legitimacy: row.get(11)?,
                midwife: row.get(12)?,
                godparents: row.get(13)?,
                priest: row.get(14)?,
                notes: row.get(15)?,
                created_at: row.get(16)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut people = Vec::new();
    for person in rows {
        people.push(person.map_err(|e| e.to_string())?);
    }
    Ok(people)
}

#[tauri::command]
fn create_person(app: tauri::AppHandle, person: NewPerson) -> Result<Person, String> {
    let conn = open_db(&app)?;
    conn.execute(
        r#"
        INSERT INTO persons (
          last_name, first_name, middle_name, sex, birth_date, birth_place,
          father, mother, residence, occupation, legitimacy, midwife,
          godparents, priest, notes
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
        "#,
        params![
            person.last_name,
            person.first_name,
            person.middle_name,
            person.sex,
            person.birth_date,
            person.birth_place,
            person.father,
            person.mother,
            person.residence,
            person.occupation,
            person.legitimacy,
            person.midwife,
            person.godparents,
            person.priest,
            person.notes
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    let mut stmt = conn
        .prepare(
            r#"
            SELECT id, last_name, first_name, middle_name, sex, birth_date,
                   birth_place, father, mother, residence, occupation,
                   legitimacy, midwife, godparents, priest, notes, created_at
            FROM persons
            WHERE id = ?1
            "#,
        )
        .map_err(|e| e.to_string())?;
    let person = stmt
        .query_row([id], |row| {
            Ok(Person {
                id: row.get(0)?,
                last_name: row.get(1)?,
                first_name: row.get(2)?,
                middle_name: row.get(3)?,
                sex: row.get(4)?,
                birth_date: row.get(5)?,
                birth_place: row.get(6)?,
                father: row.get(7)?,
                mother: row.get(8)?,
                residence: row.get(9)?,
                occupation: row.get(10)?,
                legitimacy: row.get(11)?,
                midwife: row.get(12)?,
                godparents: row.get(13)?,
                priest: row.get(14)?,
                notes: row.get(15)?,
                created_at: row.get(16)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(person)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_people, create_person])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
