use crate::db::Conn as DbConn;
// #![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket_contrib::json::Json;
use crate::models::portfolio::{Project, NewProject};
use serde_json::Value;

// GET AT PROJECTS
#[get("/projects", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
	let projects = Project::all(&conn);

	Json(json!({
		"status": 200,
		"result": projects,
	}))
}

// POST ONE PROJECT
#[post("/projects", format = "application/json", data = "<new_project>")]
pub fn new(conn: DbConn, new_project: Json<NewProject>) -> Json<Value> {
	Json(json!({
		"status": Project::insert(new_project.into_inner(), &conn),
		"result": Project::all(&conn).first(),
	}))
}

// UPDATE ONE PROJECT
#[put("/projects/<id>", format = "application/json", data = "<project>")]
pub fn update(conn: DbConn, id: i32, project: Json<NewProject>) -> Json<Value> {
	let status = if Project::update_by_id(id, &conn, project.into_inner()) {
		200
	} else { 404 };

	Json(json!({
		"status": status,
		"result": null,
	}))
}

// DELETE ONE PROJECT
#[delete("/projects/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
	let status = if Project::delete_by_id(id, &conn) {
		200
	} else { 404 };

	Json(json!({
		"status": status,
		"result": null,
	}))
}

// CATCH
#[catch(404)]
pub fn not_found() -> Json<Value> {
	Json(json!({
		"status": "error",
		"reason": "Resource was not found"
	}))
}