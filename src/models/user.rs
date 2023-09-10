use crate::models::establish_connection;
use crate::schema::users;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn find(user_id: i32) -> Result<User, diesel::result::Error> {
        let connection = &mut establish_connection();

        let user: User = users::dsl::users
            .filter(users::id.eq(user_id))
            .first(connection)?;

        Ok(user)
    }

    pub fn all() -> Result<Vec<User>, diesel::result::Error> {
        let connection = &mut establish_connection();

        let users = users::dsl::users.limit(30).load::<User>(connection)?;

        Ok(users)
    }

    pub fn create(name: &str) -> Result<User, diesel::result::Error> {
        let connection = &mut establish_connection();

        #[derive(Insertable)]
        #[table_name = "users"]
        pub struct NewUser<'a> {
            name: &'a str,
        }
        let new_user = NewUser { name: name };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)?;

        // 保存したレコードを返す
        let user: User = users::dsl::users
            .order(users::id.desc())
            .first::<User>(connection)?;

        Ok(user)
    }
}
