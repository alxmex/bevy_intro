use bevy::prelude::*;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .run();
}

fn hello_world(){
    println!("Hello World");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Alexander Mehks".to_string()));
}

fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>){
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}




