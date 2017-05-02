use graphql::context::*;
use database::models::*;
use uuid::Uuid;

pub struct QueryRoot {}
pub struct MutationRoot {}

graphql_object!(QueryRoot: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field talk(
        &executor
        id: String as "id of the talk"
    ) -> Option<&Talk> {
        use diesel::prelude::*;
        use database::models::*;
        use database::schema::talks::dsl::*;

        let variables = executor.variables();
        println!("{:?}", variables);
        let talk_id = match variables.get("id") {
            Some(value) => {
                match Uuid::parse_str(&value.to_string()[..]) {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Couldn't parse UUID");
                        return None
                    }
                }
            },
            _ => return None
        };
        let context = executor.context();
        let ref database = context.database;
        let results = talks
            .filter(published.eq(true))
            .filter(id.eq(talk_id))
            .load::<Talk>(&**database)
            .expect("Error loading talk");

        None
    }

    field talks(&executor) -> Vec<Talk> {
        use diesel::prelude::*;
        use database::models::*;
        use database::schema::talks::dsl::*;

        let context = executor.context();
        let ref database = context.database;
        let results = talks
            .filter(published.eq(true))
            .load::<Talk>(&**database)
            .expect("Error loading talks");

        results
    }
});

graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    description: "The root mutation object of the schema"
});

graphql_object!(Talk: Context as "Talk" |&self| {
    description: "Represents a talk or lecture"

    field id() -> String {
        self.id.to_string()
    }

    field title() -> String {
        self.title.to_string()
    }

    field description() -> String {
        self.description.to_string()
    }

    field kind() -> String {
        self.kind.to_string()
    }
});
