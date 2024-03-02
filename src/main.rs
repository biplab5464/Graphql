use juniper::{graphql_object,EmptySubscription,FieldResult,GraphQLEnum,
                GraphQLInputObject,GraphQLObject,ScalarValue};


#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star wars univese")]
struct Human{
    id : String,
    name : String,
    appears_in : Vec<Episode>,
    home_planet : String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHunman{
    name : String,
    appears_in: Vec<Episode>,
    home_planet : String,
}

fn main() {
    println!("Hello, world!");
}
