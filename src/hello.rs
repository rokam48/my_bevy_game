use bevy::prelude::*;
pub struct HelloPlugin;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}
#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}", name.0);
        }
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

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Albert Einstein".to_string())));
    commands.spawn((Person, Name("Bilbo Baggins".to_string())));
    commands.spawn((Person, Name("Cillian Murphy".to_string())));
}
