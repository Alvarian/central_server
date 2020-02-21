use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schemas::portfolio::projects;
use projects::dsl::projects as all_projects;


#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Project {
	pub id: i32,
	pub app_type: String,
	pub deployed_url: String,
	pub description: String,
	pub game_file: String,
	pub git_url: String,
	pub icon_file: String,
	pub style_file: String,
	pub title: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "projects"]
pub struct NewProject {
	pub app_type: String,
	pub deployed_url: String,
	pub description: String,
	pub game_file: String,
	pub git_url: String,
	pub icon_file: String,
	pub style_file: String,
	pub title: String,
}

impl Project {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Project> {
        all_projects
            .find(id)
            .load::<Project>(conn)
            .expect("Error loading book")
    }

	pub fn all(conn: &PgConnection) -> Vec<Project> {
		all_projects
			.order(projects::id.desc())
			.load::<Project>(conn)
			.expect("error loading the projects")
	}

	pub fn update_by_id(id: i32, conn: &PgConnection, project: NewProject) -> bool {
		use crate::schemas::portfolio::projects::dsl::{
			style_file as s, 
			game_file as g, 
			description as d, 
			title as t,
			app_type as a,
			deployed_url as u,
			icon_file as i,
			git_url as r,
		};

		let NewProject {
			title,
			deployed_url,
			game_file,
			style_file,
			git_url,
			icon_file,
			app_type,
			description,
		} = project;

		diesel::update(all_projects.find(id))
			.set( ( 
				s.eq(style_file), 
				g.eq(game_file), 
				d.eq(description), 
				t.eq(title),
				a.eq(app_type),
				u.eq(deployed_url),
				i.eq(icon_file),
				r.eq(git_url),
			) )
			.get_result::<Project>(conn)
			.is_ok()
	}

	pub fn insert(project: NewProject, conn: &PgConnection) -> bool {
		diesel::insert_into(projects::table)
			.values(&project)
			.execute(conn)
			.is_ok()
	}

	pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
		if Project::show(id, conn).is_empty() {
			return false;
		};

		diesel::delete(all_projects.find(id))
			.execute(conn)
			.is_ok()
	}
}