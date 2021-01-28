use crate::data::*;
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::pg::PgConnection;

sql_function!(crypt, crypt_t, (password: Text, salt: Text) -> Text);
sql_function!(gen_salt, gen_salt_t, (_type: Text) -> Text);

pub fn hashed_insert_user(user: NewUser, c: &PgConnection) -> Result<SqlUser, diesel::result::Error> {
	use crate::schema::users::dsl::*;
	
	if user.password.len() > 72 {
		panic!("Password more than 72 bytes and is not supported");
	}

	diesel::insert_into(users)
		.values((name.eq(user.name),
			username.eq(user.username),
			password.eq(crypt(user.password, gen_salt("bf"))),
			email.eq(user.email)))
		.get_result::<SqlUser>(c)
}

pub fn authorize(pass: &str, mail: &str, c: &PgConnection) -> SqlUser {
	use crate::schema::users::dsl::*;

	if pass.len() > 72 {
		panic!("Password too long");
	}
	
	users.filter(email.eq(mail))
		.filter(password.eq(crypt(pass, password)))
		.load::<SqlUser>(c)
		.expect("Error authenticating")[0].to_owned()
}
