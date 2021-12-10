#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;

use self::rocket_hello::*;
use self::models::{User, NewUser, UpdateUser, Meet, NewMeet, UpdateMeet};
use self::diesel::prelude::*;
use std::result::Result;
use rocket::http::Status;
use rocket_hello::schema::users::dsl::*;
use rocket_hello::schema::users::dsl;
use rocket_hello::schema::meets::dsl::*;
use rocket_hello::schema::users::dsl as dslm;
use rocket_hello::connection;
use connection::DbConn;
use log::{warn, error};
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

#[macro_use] extern crate rocket;
extern crate rocket_hello;
extern crate diesel;

//------------------ methods users ------------------------

#[get("/users/<id_val>")]
fn index(id_val: i32, connection: DbConn) -> res!(User) {
    let results = users.find(id_val)
        .get_result::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[get("/users")]
fn list(connection: DbConn) -> res_vec!(User) {
    let results = users.load::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[post("/users/<id_val>", format = "application/json", data = "<user>")]
fn update(id_val: i32, user: Json<UpdateUser>, connection: DbConn) -> res!(User) {
    let results = diesel::update(users)
        .filter(dsl::id.eq(id_val))
        .set(&user.0)
        .get_result::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[post("/users", format = "application/json", data = "<user>")]
fn create(user: Json<NewUser>, connection: DbConn) -> res!(User) {
    let results = diesel::insert_into(users)
        .values(&user.0)
        .get_result::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[delete("/users/<id_val>")]
fn delete(id_val: i32, conn: DbConn) -> Status {
    let result = diesel::delete(users)
        .filter(dsl::id.eq(id_val))
        .execute(&*conn);

    match result {
        Ok(val) => if val > 0 { Status::Ok } else { Status::NotFound },
        Err(e) => on_error(&e)
    }
}

// -----------------------Methods Meets --------------------------------

#[get("/meets/<id_val>")]
fn indexm(id_val: i32, connection: DbConn) -> res!(Meet) {
    let results = meets.find(id_val)
        .get_result::<Meet>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[get("/meets")]
fn listm(connection: DbConn) -> res_vec!(Meet) {
    let results = meets.load::<Meet>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}
/*
#[post("/meets/<id_val>", format = "application/json", data = "<meet>")]
fn updatem(id_val: i32, meet: Json<UpdateMeet>, connection: DbConn) -> res!(Meet) {
    let results = diesel::update(meets)
        .filter(dslm::id.eq(id_val))
        .set(&meet.0)
        .get_result::<Meet>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}
*/
#[post("/meets", format = "application/json", data = "<meet>")]
fn createm(meet: Json<NewMeet>, connection: DbConn) -> res!(Meet) {
    let results = diesel::insert_into(meets)
        .values(&meet.0)
        .get_result::<Meet>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

/*#[delete("/meets/<id_val>")]
fn deletem(id_val: i32, conn: DbConn) -> Status {
    let result = diesel::delete(meets)
        .filter(dslm::id.eq(id_val))
        .execute(&*conn);

    match result {
        Ok(val) => if val > 0 { Status::Ok } else { Status::NotFound },
        Err(e) => on_error(&e)
    }
}*/

// ---------------------- Errors -----------------------------------------

fn on_error(e: &Error) -> Status {
    match e {
        Error::NotFound => {
            warn!("Entity not found {}", e);
            Status::NotFound
        },

        _ => {
            error!("Error during request processing {}", e);
            Status::InternalServerError
        }
    }
}

//----------------------- Main -------------------------------------------------

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![index, create, list, update, delete, indexm, createm, listm])//,updatem, deletem])
        .launch(); 
}
