use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Ash".to_string()));
    commands.spawn().insert(Person).insert(Name("Billy".to_string()));
    commands.spawn().insert(Person).insert(Name("Clemens".to_string()));
}

fn hello_world() {
    println!("hello world");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}