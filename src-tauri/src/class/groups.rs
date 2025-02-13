use crate::db::AppState;
use futures::TryStreamExt; // Para poder usar try_next() en los streams
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Row;

use crate::class::subjects::Subject;

/// Estructura de un grupo
/// Se utiliza para mapear los datos del grupo de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Option<i16>,
    pub grade: i16,
    pub group: String,
    pub career: Option<String>,
    pub students: Option<i16>,
    // pub subjects: Option<Vec<Subject>>,
}

/// Funcion para crear un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `grade` - Grado
/// * `group` - Grupo
/// * `career` - Carrera (Opcional)
/// * `students` - Cantidad de alumnos (Opcional)
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn create_group(
    pool: tauri::State<'_, AppState>,
    grade: i16,
    group: String,
    career: Option<String>,
    students: Option<i16>,
    subjects: Option<Vec<Subject>>,
) -> Result<(), String> {
    let group_id: i16 = sqlx::query_scalar(
        r#"
        INSERT INTO groups (grade, "group", career, students)
        VALUES (?1, ?2, ?3, ?4)
        RETURNING id
    "#,
    )
    .bind(grade)
    .bind(group)
    .bind(career)
    .bind(students)
    .fetch_one(&pool.db)
    .await
    .map_err(|e| format!("Failed to create group, error: {}", e))?;

    if let Some(subjects) = subjects {
        for subject in subjects {
            // Checa si existe la materia en el grupo, regresa el id si se encuentra
            let check_groups: Option<i16> = sqlx::query_scalar(
                "
                SELECT 1 FROM groups_subjects
                WHERE group_id = ?1 AND subject_id = ?2
                ",
            )
            .bind(group_id)
            .bind(subject.id)
            .fetch_optional(&pool.db)
            .await
            .map_err(|e| format!("Error checking if subject exists on group table: {}", e))?;

            // Si no se encuentra lo asigna
            if check_groups.is_none() {
                sqlx::query("INSERT INTO groups_subjects (group_id, subject_id) VALUES (?1, ?2)")
                    .bind(group_id)
                    .bind(subject.id)
                    .fetch_optional(&pool.db)
                    .await
                    .map_err(|e| format!("Error assigning subject to group: {}", e))?;
            }
        }
    }

    Ok(())
}

/// Funcion para crear varios grupos a la vez
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `groups` - Vector de grupos
/// Retorna Ok() si todo sale exitoso de lo contrario manda un mensaje con el error
#[tauri::command]
pub async fn create_groups(
    pool: tauri::State<'_, AppState>,
    groups: Vec<Group>,
) -> Result<(), String> {
    let mut tx = pool
        .db
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction! {}", e))?;

    for g in groups {
        sqlx::query(
            r#"INSERT INTO groups(grade, "group", career, students) VALUES (?1, ?2, ?3, ?4)"#,
        )
        .bind(g.grade)
        .bind(g.group)
        .bind(g.career)
        .bind(g.students)
        .execute(&mut tx)
        .await
        .map_err(|e| format!("Error creating the group, error: {}", e))?;
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    println!("Groups created successfully");
    Ok(())
}

/// Funcion para obtener todos los grupos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con todos los grupos
/// Se llama desde la interfaz de usuario para obtenerlos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_groups(
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<(Group, Vec<Subject>)>, String> {
    let groups: Vec<Group> = sqlx::query_as::<_, Group>("SELECT * FROM groups")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    // Checar si hay alguna materia asignada al grupo
    let mut groups_subjects: Vec<(Group, Vec<Subject>)> = Vec::new();
    for group in groups {
        // Obtener materias
        let subject_id: Vec<i16> =
            sqlx::query("SELECT subject_id FROM groups_subjects WHERE group_id = ?1")
                .bind(group.id)
                .fetch(&pool.db)
                .map_ok(|row| row.get::<i16, _>(0)) // Obtener el/los ID de la materia
                .try_collect()
                .await
                .map_err(|e| format!("Failed to get subject id from database: {}", e))?;

        let mut subjects: Vec<Subject> = Vec::new();
        for id in subject_id {
            let subject: Subject =
                sqlx::query_as::<_, Subject>("SELECT * FROM subjects WHERE id = ?1")
                    .bind(id)
                    .fetch_one(&pool.db)
                    .await
                    .map_err(|e| format!("Failed to get subject class: {}", e))?;

            println!("Subject: {:?}", subject);
            subjects.push(subject);
        }

        groups_subjects.push((group, subjects));
    }

    Ok(groups_subjects)
}

/// Funcion para eliminar un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID del grupo:
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar un grupo
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_group(pool: tauri::State<'_, AppState>, id: i16) -> Result<(), String> {
    sqlx::query("DELETE FROM groups WHERE id = ?1")
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete group: {}", e))?;

    Ok(())
}

/// Funcion para eliminar varios grupos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `ids` - Vector con los ID del grupo:
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar varios grupos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_groups(pool: tauri::State<'_, AppState>, ids: Vec<i16>) -> Result<(), String> {
    for i in ids {
        delete_group(pool.clone(), i).await?;
    }

    Ok(())
}

/// Funcion para actualizar un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID del grupo
/// * `grade` - Grado
/// * `group` - Grupo
/// * `career` - Carrera (Opcional)
/// * `students` - Cantidad de alumnos (Opcional)
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para actualizar un grupo
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn update_group(
    pool: tauri::State<'_, AppState>,
    id: i16,
    grade: i16,
    group: String,
    career: Option<String>,
    students: Option<i16>,
    subjects: Option<Vec<Subject>>,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE groups SET grade = ?1, "group" = ?2, career = ?3, students = ?4 WHERE id = ?5"#,
    )
    .bind(grade)
    .bind(group)
    .bind(career)
    .bind(students)
    .bind(id)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to update group: {}", e))?;

    if let Some(subjects) = subjects {
        // Eliminar las materias del grupo si existian
        sqlx::query("DELETE FROM groups_subjects WHERE group_id = ?1")
            .bind(id)
            .execute(&pool.db)
            .await
            .map_err(|e| format!("Failed to delete group subject: {}", e))?;
        for subject in subjects {
            // Agrega materia al grupo
            sqlx::query("INSERT INTO groups_subjects (group_id, subject_id) VALUES (?1, ?2)")
                .bind(id)
                .bind(subject.id)
                .fetch_optional(&pool.db)
                .await
                .map_err(|e| format!("Failed to assign the subject to existed group: {}", e))?;
        }
    }

    Ok(())
}
