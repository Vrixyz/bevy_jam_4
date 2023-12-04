use crate::simple_mouse::MouseWorldPosition;

use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::ItemType;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShapesAmount>();
        app.add_systems(
            Update,
            (
                clear_build_requests,
                click_get_out,
                apply_deferred,
                (
                    verify_empty_space,
                    // TODO: add more checks
                    apply_deferred,
                    react_to_build,
                )
                    .run_if(component_exist::<BuildRequest>)
                    .chain(),
            )
                .chain(),
        );
    }
}

#[derive(Resource, Default)]
struct ShapesAmount(usize);

#[derive(Component)]
struct BuildRequest {
    pub inventory: Entity,
    pub item: Entity,
    pub position: Vec2,
}

#[derive(Component)]
enum RefusedBuild {
    NotEnoughPlace,
}

fn component_exist<T: Component>(q: Query<Entity, With<T>>) -> bool {
    q.iter().next().is_some()
}

fn click_get_out(
    mut commands: Commands,
    selection: Query<&crate::Selection>,
    mut q_inventory: Query<(Entity, &mut inventory::Inventory<super::ItemType>)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_position_world: Res<MouseWorldPosition>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        let selection = selection.single();
        for mut i in q_inventory.iter_mut() {
            if selection.inventories[selection.selected_index] != i.0 {
                continue;
            }
            let first = i.1.items.front().unwrap();

            commands.spawn(BuildRequest {
                inventory: i.0,
                item: *first,
                position: mouse_position_world.0,
            });
        }
    }
}

fn verify_empty_space(mut commands: Commands, q_requests: Query<(Entity, &BuildRequest)>) {
    for br in q_requests.iter() {
        info!("build at: {}", &br.1.position.x);
        if !(-200f32..300f32).contains(&br.1.position.x)
            || !(-100f32..250f32).contains(&br.1.position.y)
        {
            info!("forbidden");
            commands.entity(br.0).insert(RefusedBuild::NotEnoughPlace);
        }
    }
}

// TODO use cmponents and check everything ok
fn react_to_build(
    mut commands: Commands,
    mut q_inventory: Query<&mut inventory::Inventory<super::ItemType>>,
    build_events: Query<&BuildRequest, Without<RefusedBuild>>,
    mut q_transform: Query<&mut Transform>,
    mut rng: ResMut<crate::RandomDeterministic>,
    mut shapes_amount: ResMut<ShapesAmount>,
) {
    for event in build_events.iter() {
        let mut inventory = q_inventory.get_mut(event.inventory).unwrap();
        let item_index = inventory
            .items
            .iter()
            .position(|i| *i == event.item)
            .unwrap();
        inventory.items.remove(item_index);
        shapes_amount.0 += 1;
        q_transform.get_mut(event.item).unwrap().translation =
            event.position.extend(shapes_amount.0 as f32 * 0.000001f32);

        let item_type = super::get_random_item(&mut rng);
        inventory.items.push_back(commands.spawn(item_type).id());
    }
}

fn clear_build_requests(mut commands: Commands, build_events: Query<Entity, With<BuildRequest>>) {
    for e in build_events.iter() {
        commands.entity(e).despawn();
    }
}
