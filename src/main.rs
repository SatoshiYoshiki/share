#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use schema::twilio_phone_number::{self, dsl};

mod schema {
    include!("schema.rs");
}

mod model;

#[derive(Insertable, Queryable,  Debug, PartialEq)]
#[table_name = "twilio_phone_number"]
struct PhoneNumberRecord {
    pub phone_number: String,
}

fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e));
    /*
    let _ = diesel::insert_into(dsl::twilio_phone_number)
        .values(&Translation {
            word_id: 1,
            translation_id: 1,
            language: model::Language::En,
        })
        .execute(&conn);
    */
    let t = dsl::twilio_phone_number
        .select(dsl::phone_number)
        .get_results::<PhoneNumberRecord>(&conn)
        .expect("select");
    println!("{:?}", t);
}
