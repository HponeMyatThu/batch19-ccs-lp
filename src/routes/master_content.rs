use crate::db::sqlite::AppState;
use crate::models::master_content::MasterContent;
use actix_web::{HttpResponse, Responder, web};
use rusqlite::params;
use serde_json::json;

// Initialize routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/master_content")
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_one))
            .route("", web::post().to(create))
            .route("", web::put().to(update))
            .route("/{id}", web::delete().to(delete)),
    );
}

// Get all visible content
async fn get_all(data: web::Data<AppState>) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM master_content WHERE visible = 1 ORDER BY display_order ASC")
        .unwrap();

    let items = stmt
        .query_map([], |row| {
            Ok(MasterContent {
                id: Some(row.get(0)?),
                page_name: row.get(1)?,
                section_name: row.get(2)?,
                lang: row.get(3)?,
                content_type: row.get(4)?,
                content: row.get(5)?,
                visible: Some(row.get(6)?),
                display_order: Some(row.get(7)?),
            })
        })
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<MasterContent>>();

    HttpResponse::Ok().json(items)
}

// Get one by ID
async fn get_one(info: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM master_content WHERE id=?1")
        .unwrap();
    let item = stmt.query_row([*info], |row| {
        Ok(MasterContent {
            id: Some(row.get(0)?),
            page_name: row.get(1)?,
            section_name: row.get(2)?,
            lang: row.get(3)?,
            content_type: row.get(4)?,
            content: row.get(5)?,
            visible: Some(row.get(6)?),
            display_order: Some(row.get(7)?),
        })
    });

    match item {
        Ok(i) => HttpResponse::Ok().json(i),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

// Create
async fn create(item: web::Json<MasterContent>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO master_content (page_name, section_name, lang, content_type, content, visible, display_order)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            item.page_name,
            item.section_name,
            item.lang,
            item.content_type,
            item.content,
            item.visible.unwrap_or(1),
            item.display_order.unwrap_or(0),
        ],
    ).unwrap();
    HttpResponse::Ok().json(json!({"status": "created"}))
}

// Update
async fn update(item: web::Json<MasterContent>, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = item.id {
        let conn = data.conn.lock().unwrap();
        conn.execute(
            "UPDATE master_content SET page_name=?1, section_name=?2, lang=?3, content_type=?4, content=?5, visible=?6, display_order=?7, updated_at=CURRENT_TIMESTAMP WHERE id=?8",
            params![
                item.page_name,
                item.section_name,
                item.lang,
                item.content_type,
                item.content,
                item.visible.unwrap_or(1),
                item.display_order.unwrap_or(0),
                id
            ],
        ).unwrap();
        HttpResponse::Ok().json(json!({"status": "updated"}))
    } else {
        HttpResponse::BadRequest().body("ID required")
    }
}

// Delete
async fn delete(info: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    conn.execute("DELETE FROM master_content WHERE id=?1", params![*info])
        .unwrap();
    HttpResponse::Ok().json(json!({"status": "deleted"}))
}
