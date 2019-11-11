use crate::api;


pub mod kill;
pub mod killmail;
pub mod attacker;
pub mod victim;
pub mod item;
pub mod object;
pub mod category;
pub mod stargate;

pub use diesel::sqlite::SqliteConnection as Connection;

pub type Bool = bool;
pub type Integer = i32;
pub type OptInteger = Option<Integer>;
pub type OptString = Option<String>;
pub type Float = f32;
pub type Hash = String;
pub type Date = chrono::NaiveDate;
pub type DateTime = chrono::NaiveDateTime;
pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;


pub struct DB;
impl DB {
    pub fn connection() -> Connection {
        use crate::diesel::Connection;
        let url = std::env::var("DATABASE_URL").expect("DB_URL environment variable required");
        Connection::establish(&url).expect(&format!("Error connection to {}", url))
    }

    // pub fn load_kills(conn: &Connection, date: &Date) -> QueryResult<Vec<kill::Kill>> {
    //     use diesel::prelude::*;
    //     use super::schema::kills::dsl as table;
    //     table::kills.filter(table::killmail_date.eq(&date)).load(conn)
    // }

    // pub fn save_kills(conn: &Connection, kills: &Vec<kill::Kill>) -> QueryResult<usize> {
    //     use diesel::prelude::*;
    //     use super::schema;
    //     diesel::insert_into(schema::kills::table).values(kills).execute(conn)
    // }

    // pub fn get_saved_killmails(conn: &Connection, date: &Date) -> HashSet<Integer> {
    //     let start =date.and_hms(0, 0, 0);
    //     let end =date.and_hms(23, 59, 59);

    //     use diesel::prelude::*;
    //     use std::iter::FromIterator;
    //     use super::schema::killmails::dsl as table;
    //     let vector = table::killmails
    //                 .filter(table::killmail_time.between(&start, &end))
    //                 .select(table::killmail_id).load(conn).unwrap();
    //     HashSet::from_iter(vector.iter().cloned())
    // }

    // pub fn save_all(conn: &Connection, killmail: &Vec<api::Killmail>) -> QueryResult<()> {
    //     use super::schema;
    //     use diesel::connection::Connection;
    //     use diesel::RunQueryDsl;

    //     let mut killmails = Vec::new();
    //     let mut victims = Vec::new();
    //     let mut attackers = Vec::new();
    //     let mut items = Vec::new();

    //     for killmail in killmail.iter() {
    //         victims.push(victim::Victim::from(killmail));
    //         killmails.push(killmail::from(killmail));
    //         attackers.append(&mut get_attackers(killmail));
    //         items.append(&mut get_items(killmail));
    //     }

    //     conn.transaction::<_, _, _>(|| {
    //         diesel::insert_into(schema::killmails::table).values(&killmails).execute(conn)?;
    //         diesel::insert_into(schema::attackers::table).values(&attackers).execute(conn)?;
    //         diesel::insert_into(schema::victims::table).values(&victims).execute(conn)?;
    //         diesel::insert_into(schema::items::table).values(&items).execute(conn)?;
    //         Ok(())
    //     })
    // }
}

pub struct KillmailsApi;
impl KillmailsApi {

    pub fn save(conn: &Connection, killmail: &api::Killmail) -> QueryResult<()> {
        if !KillmailsApi::exist(conn, killmail.killmail_id) {
            KillmailsApi::do_save(conn, killmail)
        } else {
            Ok(())
        }
    }

    fn do_save(conn: &Connection, killmail: &api::Killmail) -> QueryResult<()> {
        use super::schema;
        use diesel::connection::Connection;
        use diesel::RunQueryDsl;

        conn.transaction::<_, _, _>(|| {
            diesel::insert_into(schema::killmails::table)
                   .values(&killmail::Killmail::from(killmail))
                   .execute(conn)?;
            diesel::insert_into(schema::victims::table)
                   .values(&victim::Victim::from(killmail))
                   .execute(conn)?;
            diesel::insert_into(schema::attackers::table)
                   .values(&get_attackers(killmail))
                   .execute(conn)?;
            diesel::insert_into(schema::items::table)
                   .values(&get_items(killmail))
                   .execute(conn)?;
            Ok(())
        })
    }

    // pub fn load(conn: &Connection, id: &Integer) -> QueryResult<object::Object>  {
    //     use diesel::prelude::*;
    //     use crate::schema::objects::dsl as table;
    //     table::objects.filter(table::object_id.eq(id)).first(conn)
    // }

    pub fn exist(conn: &Connection, killmail_id: Integer) -> bool {
        use diesel::prelude::*;
        use super::schema::killmails::dsl as table;
        table::killmails.find(killmail_id).select(table::killmail_id).first(conn) == Ok(killmail_id)
    }
}


pub struct CategoriesApi;
impl CategoriesApi {
    pub fn save(conn: &Connection, object: &api::object::Object) -> QueryResult<bool>  {
        use super::schema;
        use crate::schema::categories::dsl::*;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::ExpressionMethods;
        diesel::insert_into(schema::categories::table)
                   .values(category_name.eq(&object.category))
                   .execute(conn).and_then(|count| Ok(1 == count))
    }

    pub fn find(conn: &Connection, name: &String) -> QueryResult<category::Category>  {
        use diesel::prelude::*;
        use crate::schema::categories::dsl as table;
        table::categories.filter(table::category_name.eq(name)).first(conn)
    }
}

pub struct ObjectsApi;
impl ObjectsApi {
    pub fn save(conn: &Connection, object: &api::object::Object) -> QueryResult<bool>  {
        use super::schema;
        use diesel::RunQueryDsl;
        let category = match CategoriesApi::find(conn, &object.category) {
            Ok(category) => {
                Ok(category)
            },
            Err(diesel::result::Error::NotFound) => {
                CategoriesApi::save(conn, &object)?;
                CategoriesApi::find(conn, &object.category)
            },
            Err(err) => {
                error!("Was not able to save object: {}", err);
                Err(err)
            },
        }?;

        let data = object::Object::new(object.id, category.category_id, object.name.clone());
        diesel::insert_into(schema::objects::table).values(&data).execute(conn).and_then(|count| Ok(1 == count))
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<object::Object>  {
        use diesel::prelude::*;
        use crate::schema::objects::dsl as table;
        table::objects.filter(table::object_id.eq(id)).first(conn)
    }

    pub fn exist(conn: &Connection, id: &Integer) -> bool {
        use diesel::prelude::*;
        use crate::schema::objects::dsl as table;
        table::objects.find(id).select(table::object_id).first(conn) == Ok(*id)
    }
}

fn get_attackers(killmail: &api::Killmail) -> Vec<attacker::Attacker> {
    let mut result = Vec::new();
    for attacker in &killmail.attackers {
        let mut obj = attacker::Attacker::from(attacker);
        obj.killmail_id = killmail.killmail_id;
        result.push(obj);
    }
    return result;
}

fn get_items(killmail: &api::Killmail) -> Vec<item::Item> {
    let mut result = Vec::new();
    if let Some(ref items) = &killmail.victim.items {
        for item in items {
            let mut obj = item::Item::from(item);
            obj.killmail_id = killmail.killmail_id;
            result.push(obj);
            if let Some(ref subitems) = item.items {
                for item in subitems {
                    let mut obj = item::Item::from(item);
                    obj.killmail_id = killmail.killmail_id;
                    result.push(obj);
                }
            }
        }
    }
    return result;
}
