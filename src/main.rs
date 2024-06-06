use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

fn hello_world() {
    println!("Hello Everyone!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Albert Einstein".to_string())));
    commands.spawn((Person, Name("Bilbo Baggins".to_string())));
    commands.spawn((Person, Name("Cillian Murphy".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Cillian Murphy" {
            name.0 = "Christian Bale".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}
