use bevy::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
	commands.spawn((Person, Name("Harry Potter".to_string()), Position { x: 120., y: 345. }));
	commands.spawn((Person, Name("Ron Weasley".to_string()), Position { x: 100., y: 600. }));
}

#[derive(Resource)]
struct PositionTimer(Timer);

#[derive(Resource)]
struct GreetTimer(Timer);

fn print_position_system(time: Res<Time>, mut timer: ResMut<PositionTimer>, query: Query<(&Name, &Position), With<Person>>) {
	if timer.0.tick(time.delta()).just_finished() {
		for (name, position) in &query {
			println!("{}'s position: ({}, {})", name.0, position.x, position.y);
		}
	}
}

fn greet_people_system(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
	if timer.0.tick(time.delta()).just_finished() {
		for name in &query {
			println!("hello {}", name.0);
		}
	}
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(PositionTimer(Timer::from_seconds(4.0, TimerMode::Repeating)))
			.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
			.add_systems(Startup, add_people)
			.add_systems(Update, (greet_people_system, print_position_system));

	}
}

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, HelloPlugin))
		.run();
}

