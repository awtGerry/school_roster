use crate::db::AppState;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Row;

/// Estructura simple de un profesor, solo contiene el ID, el nombre y el primer apellido
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleTeacher {
    pub id: Option<i16>,
    pub name: String,
    pub father_lastname: String,
}

/// Estructura de un profesor
/// Se utiliza para mapear los datos de un profesor de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Teacher {
    pub id: Option<i16>,
    pub name: String,
    pub father_lastname: String,
    pub mother_lastname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub degree: Option<String>,
    pub commisioned_hours: Option<i16>, // Total de horas
    pub active_hours: Option<i16>,      // Horas activas en el programa
    pub performance: Option<i16>,       // Desempeño
}

/// Funcion para agregar un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `name` - Nombre del profesor
/// * `father_lastname` - Apellido paterno del profesor
/// * `mother_lastname` - Apellido materno del profesor (opcional puede ser nulo)
/// * `email` - Correo electronico del profesor (opcional puede ser nulo)
/// * `phone` - Telefono del profesor (opcional puede ser nulo)
/// * `degree` - Grado academico del profesor (opcional puede ser nulo)
/// * `commisioned_hours` - Total de horas comisionadas del profesor (opcional puede ser nulo)
/// * `active_hours` - Total de horas activas del profesor (opcional puede ser nulo)
/// * `performance` - Desempeño del profesor (opcional puede ser nulo)
/// * `subjects` - Materias que imparte el profesor (opcional puede ser nulo)
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn add_teacher(
    pool: tauri::State<'_, AppState>,
    name: String,
    father_lastname: String,
    mother_lastname: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    degree: Option<String>,
    commisioned_hours: Option<i16>,
    active_hours: Option<i16>,
    performance: Option<i16>,
    subjects: Option<Vec<i16>>,
) -> Result<(), String> {
    let teacher_id: i64 = sqlx::query_scalar(
        "
        INSERT INTO teachers (
            name,
            father_lastname,
            mother_lastname,
            email,
            phone,
            degree,
            commisioned_hours,
            active_hours,
            performance
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        RETURNING id
    ",
    )
    .bind(name)
    .bind(father_lastname)
    .bind(mother_lastname)
    .bind(email)
    .bind(phone)
    .bind(degree)
    .bind(commisioned_hours)
    .bind(active_hours)
    .bind(performance)
    .fetch_one(&pool.db)
    .await
    .map_err(|e| format!("Failed to create teacher: {}", e))?;

    if let Some(subjects) = subjects {
        // Agregar las materias al profesor
        for subject_id in subjects {
            let exists: Option<i16> = sqlx::query_scalar(
                "
                SELECT 1 FROM teacher_subjects
                WHERE teacher_id = ?1 AND subject_id = ?2
            ",
            )
            .bind(teacher_id)
            .bind(subject_id)
            .fetch_optional(&pool.db)
            .await
            .map_err(|e| {
                format!(
                    "Failed to check if subject is already attached to teacher: {}",
                    e
                )
            })?;

            if exists.is_none() {
                sqlx::query(
                    "
                    INSERT INTO teacher_subjects (teacher_id, subject_id)
                    VALUES (?1, ?2)",
                )
                .bind(teacher_id)
                .bind(subject_id)
                .execute(&pool.db)
                .await
                .map_err(|e| format!("Failed to attach subject to teacher: {}", e))?;
            } else {
                println!("Subject already attached to teacher");
            }
        }
    }

    Ok(())
}

/// Funcion para crear varios profesores
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher` - Clase del profesor (sin materia)
/// Se llama desde la interfaz para registrar varios profesores a la vez utilizando excel (sin materias)
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn create_teachers(
    pool: tauri::State<'_, AppState>,
    teacher: Vec<Teacher>,
) -> Result<(), String> {
    for i in teacher {
        sqlx::query(
            "
        INSERT INTO teachers (
            name, father_lastname, mother_lastname,
            email, phone, degree, commisioned_hours,
            active_hours, performance
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )
        .bind(i.name)
        .bind(i.father_lastname)
        .bind(i.mother_lastname)
        .bind(i.email)
        .bind(i.phone)
        .bind(i.degree)
        .bind(i.commisioned_hours)
        .bind(i.active_hours)
        .bind(i.performance)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error creating the teacher, error: {}", e))?;
    }

    Ok(())
}

/// Funcion para editar un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID del profesor
/// * `name` - Nombre del profesor
/// * `father_lastname` - Apellido paterno del profesor
/// * `mother_lastname` - Apellido materno del profesor (opcional puede ser nulo)
/// * `email` - Correo electronico del profesor (opcional puede ser nulo)
/// * `phone` - Telefono del profesor (opcional puede ser nulo)
/// * `degree` - Grado academico del profesor (opcional puede ser nulo)
/// * `commisioned_hours` - Total de horas comisionadas del profesor (opcional puede ser nulo)
/// * `active_hours` - Total de horas activas del profesor (opcional puede ser nulo)
/// * `performance` - Desempeño del profesor (opcional puede ser nulo)
/// * `subjects` - Materias que imparte el profesor (opcional puede ser nulo)
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn edit_teacher(
    pool: tauri::State<'_, AppState>,
    id: i16,
    name: String,
    father_lastname: String,
    mother_lastname: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    degree: Option<String>,
    commisioned_hours: Option<i16>,
    active_hours: Option<i16>,
    performance: Option<i16>,
    subjects: Option<Vec<i16>>,
) -> Result<(), String> {
    // Actualizar los datos del profesor
    sqlx::query(
        "
        UPDATE teachers
        SET
            name = ?1,
            father_lastname = ?2,
            mother_lastname = ?3,
            email = ?4,
            phone = ?5,
            degree = ?6,
            commisioned_hours = ?7,
            active_hours = ?8,
            performance = ?9
        WHERE id = ?10
    ",
    )
    .bind(name)
    .bind(father_lastname)
    .bind(mother_lastname)
    .bind(email)
    .bind(phone)
    .bind(degree)
    .bind(commisioned_hours)
    .bind(active_hours)
    .bind(performance)
    .bind(id)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to update teacher: {}", e))?;

    if let Some(subjects) = subjects {
        // Eliminar las materias del profesor
        sqlx::query("DELETE FROM teacher_subjects WHERE teacher_id = ?1")
            .bind(id)
            .execute(&pool.db)
            .await
            .map_err(|e| format!("Failed to delete teacher subjects: {}", e))?;

        // Agregar las materias al profesor
        for subject_id in subjects {
            sqlx::query(
                "
                INSERT INTO teacher_subjects (teacher_id, subject_id)
                VALUES (?1, ?2)
            ",
            )
            .bind(id)
            .bind(subject_id)
            .execute(&pool.db)
            .await
            .map_err(|e| format!("Failed to attach subject to teacher: {}", e))?;
        }
    }

    Ok(())
}

/// Funcion para obtener todos los profesores
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con todos los profesores
/// Se llama desde la interfaz de usuario para obtener todos los profesores
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_all_teachers(
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<(Teacher, Vec<i16>)>, String> {
    // Obtener todos los profesores
    let teachers: Vec<Teacher> = sqlx::query_as::<_, Teacher>("SELECT * FROM teachers")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    let mut teachers_with_subjects = Vec::new();
    for teacher in teachers {
        // Obtener las materias que imparte el profesor
        let subjects: Vec<i16> =
            sqlx::query("SELECT subject_id FROM teacher_subjects WHERE teacher_id = ?1")
                .bind(teacher.id) // Mapear el ID del profesor
                .fetch(&pool.db)
                .map_ok(|row| row.get::<i16, _>(0)) // Obtener el/los ID de la materia
                .try_collect() // Convertir el resultado en un vector
                .await
                .map_err(|e| {
                    format!(
                        "Failed to fetch subjects for teacher {}: {}",
                        teacher.name, e
                    )
                })?;

        // Agregar el profesor y sus materias al vector
        teachers_with_subjects.push((teacher, subjects));
    }

    Ok(teachers_with_subjects)
}

/// Funcion para eliminar a un profesor (y todas sus materias asignadas)
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher_id` - ID del profesor
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar a un profesor
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_teacher(
    pool: tauri::State<'_, AppState>,
    teacher_id: i16,
) -> Result<(), String> {
    sqlx::query("DELETE FROM teachers WHERE id = ?1")
        .bind(teacher_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete teacher: {}", e))?;

    sqlx::query("DELETE FROM assignments WHERE teacher_id = ?1")
        .bind(teacher_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete teacher: {}", e))?;

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
pub async fn delete_teachers(
    pool: tauri::State<'_, AppState>,
    ids: Vec<i16>,
) -> Result<(), String> {
    for i in ids {
        delete_teacher(pool.clone(), i).await?;
    }
    Ok(())
}
