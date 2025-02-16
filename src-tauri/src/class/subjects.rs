use crate::class::teachers::SimpleTeacher;
use crate::db::AppState;
use futures::TryStreamExt; // Para poder usar try_next() en los streams
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Row;

/// Estructura de una materia
/// Se utiliza para mapear los datos de una materia de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Subject {
    pub id: Option<i16>, // ID opcional cuando se crea por medio de la clase
    pub name: String,
    pub shorten: String,
    pub color: String,
    pub spec: String,
}

/// Estructura de una materia con profesor asignado
/// Se utiliza para mapear los datos de una materia de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectWithTeacher {
    pub id: i16,
    pub name: String,
    pub shorten: String,
    pub color: String,
    pub spec: String,
    pub assigned_teacher: Option<SimpleTeacher>,
}

/// Funcion para crear una materia
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `name` - Nombre de la materia
/// * `shorten` - Abreviatura de la materia
/// * `color` - Color de la materia
/// * `spec` - Especificacion de la materia
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn create_subject(
    pool: tauri::State<'_, AppState>,
    name: String,
    shorten: String,
    color: String,
    spec: String,
) -> Result<(), String> {
    sqlx::query("INSERT INTO subjects (name, shorten, color, spec) VALUES (?1, ?2, ?3, ?4)")
        .bind(name)
        .bind(shorten)
        .bind(color)
        .bind(spec)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to create subject: {}", e))?;

    println!("Subject created successfully");

    Ok(())
}

/// Funcion para crear varios elementos a la vez
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `classrooms` - Vector de grupos
/// Retorna Ok() si todo sale exitoso de lo contrario manda un mensaje con el error
#[tauri::command]
pub async fn create_subjects(
    pool: tauri::State<'_, AppState>,
    subject: Vec<Subject>,
) -> Result<(), String> {
    for i in subject {
        sqlx::query("INSERT INTO subjects (shorten, name, color, spec) VALUES (?1, ?2, ?3, ?4)")
            .bind(if i.shorten.len() <= 0 {
                i.name.to_uppercase().chars().take(3).collect()
            } else {
                i.shorten
            })
            .bind(i.name)
            .bind(i.color)
            .bind(i.spec)
            .execute(&pool.db)
            .await
            .map_err(|e| format!("Error creating the classroom, error: {}", e))?;
    }

    Ok(())
}

/// Funcion para obtener todas las materias
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con todas las materias
/// Se llama desde la interfaz de usuario para obtener todas las materias
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_subjects(pool: tauri::State<'_, AppState>) -> Result<Vec<Subject>, String> {
    let subjects: Vec<Subject> = sqlx::query_as::<_, Subject>("SELECT * FROM subjects")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(subjects)
}

/// Funcion para eliminar una materia
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID de la materia
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar una materia
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_subject(pool: tauri::State<'_, AppState>, id: i16) -> Result<(), String> {
    sqlx::query("DELETE FROM subjects WHERE id = ?1")
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete subject: {}", e))?;

    sqlx::query("DELETE FROM groups_subjects WHERE subject_id = ?1")
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete subject: {}", e))?;

    Ok(())
}

/// Funcion para eliminar varios elementos de la base de datos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `ids` - ID del elemento a eliminar
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar varios elementos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_subjects(
    pool: tauri::State<'_, AppState>,
    ids: Vec<i16>,
) -> Result<(), String> {
    for i in ids {
        delete_subject(pool.clone(), i).await?;
    }
    Ok(())
}

/// Funcion para actualizar una materia
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID de la materia
/// * `name` - Nombre de la materia
/// * `shorten` - Abreviatura de la materia
/// * `color` - Color de la materia
/// * `spec` - Especificacion de la materia
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para actualizar una materia
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn update_subject(
    pool: tauri::State<'_, AppState>,
    id: i16,
    name: String,
    shorten: String,
    color: String,
    spec: String,
) -> Result<(), String> {
    sqlx::query("UPDATE subjects SET name = ?1, shorten = ?2, color = ?3, spec = ?4 WHERE id = ?5")
        .bind(name)
        .bind(shorten)
        .bind(color)
        .bind(spec)
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to update subject: {}", e))?;

    Ok(())
}

/// Funcion para obtener materias que tengan profesores asignados
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con todas las materias que tengan profesores asignados
/// Se llama desde la interfaz de usuario para obtener todas las materias que tengan profesores asignados
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_subjects_with_teachers(
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<SubjectWithTeacher>, String> {
    let rows = sqlx::query(
        "
        SELECT
            subjects.id as subject_id,
            subjects.name as subject_name,
            subjects.shorten as subject_shorten,
            subjects.color as subject_color,
            subjects.spec as subject_spec,
            teachers.id as teacher_id,
            teachers.name as teacher_name,
            teachers.father_lastname as teacher_father_lastname
        FROM subjects
        LEFT JOIN teacher_subjects ON subjects.id = teacher_subjects.subject_id
        LEFT JOIN teachers ON teacher_subjects.teacher_id = teachers.id
    ",
    )
    .fetch_all(&pool.db)
    .await
    .map_err(|e| format!("Failed to fetch subjects with teachers: {}", e))?;

    // Manualmente mapeamos los resultados a un vector de materias
    let mut subjects_with_teachers: Vec<SubjectWithTeacher> = Vec::new();

    for row in rows {
        let teacher_id: Option<i16> = row.try_get("teacher_id").unwrap_or(None);
        let assigned_teacher: Option<SimpleTeacher> = match teacher_id {
            Some(teacher_id) => Some(SimpleTeacher {
                id: Some(teacher_id),
                name: row.try_get("teacher_name").unwrap(),
                father_lastname: row.try_get("teacher_father_lastname").unwrap(),
            }),
            None => None,
        };

        let subject = SubjectWithTeacher {
            id: row.try_get("subject_id").unwrap(),
            name: row.try_get("subject_name").unwrap(),
            shorten: row.try_get("subject_shorten").unwrap(),
            color: row.try_get("subject_color").unwrap(),
            spec: row.try_get("subject_spec").unwrap(),
            assigned_teacher,
        };

        subjects_with_teachers.push(subject);
    }

    Ok(subjects_with_teachers)
}
