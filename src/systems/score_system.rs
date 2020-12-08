use crate::entities::collision::{are_colliding, Arrival, Colliders};
use crate::entities::ship::ShipParent;
use crate::entities::TransitionFade;
use crate::resources::main_resource::MainResource;
use amethyst::core::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::core::{Time, Transform};
use amethyst::renderer::palette::Srgba;
use amethyst::ui::{Anchor, ScaleMode, UiImage, UiImagePrefab, UiTransform, UiWidget};

const DEFAULT_TIMER: f32 = 0.6;

pub struct ScoreSystem {
    fade_in_timer: f32,
}

impl Default for ScoreSystem {
    fn default() -> Self {
        ScoreSystem {
            fade_in_timer: DEFAULT_TIMER,
        }
    }
}

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, ShipParent>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Arrival>,
        Write<'s, MainResource>,
        Read<'s, Time>,
        Entities<'s>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiImage>,
        WriteStorage<'s, TransitionFade>,
    );

    fn run(
        &mut self,
        (
            colliders,
            ships,
            transforms,
            arrivals,
            mut main_resource,
            time,
            entities,
            mut ui_transforms,
            mut images,
            mut fades,
        ): Self::SystemData,
    ) {
        if main_resource.is_landed && !main_resource.victory {
            for (_ship, transform) in (&ships, &transforms).join() {
                let ship_polygon = main_resource.get_colliders_polygons_for_landing(
                    transform.translation().x,
                    transform.translation().y,
                );
                for (collider, _) in (&colliders, &arrivals).join() {
                    let struct_polygons = collider.polygons();
                    if are_colliding(&ship_polygon, struct_polygons)
                        && main_resource.collected_coin == main_resource.level_config().coin_nb
                    {
                        main_resource.victory = true;
                    }
                }
            }
        }

        if main_resource.victory {
            if self.fade_in_timer == DEFAULT_TIMER {
                let mut t = UiTransform::new(
                    String::from("developer-console-output-transform"),
                    Anchor::TopLeft,
                    Anchor::TopLeft,
                    0.,
                    0.,
                    100.,
                    1.0,
                    1.0,
                );
                t.scale_mode = ScaleMode::Percent;
                let (r, g, b, a) = Srgba::new(31. / 255., 54. / 255., 50. / 255., 0.)
                    .into_linear()
                    .into_components();

                entities
                    .build_entity()
                    .with(t, &mut ui_transforms)
                    .with(UiImage::SolidColor([r, g, b, a]), &mut images)
                    .with(TransitionFade, &mut fades)
                    .build();
            } else if self.fade_in_timer <= 0. {
                main_resource.should_go_to_next_level = true;
            } else {
                for (_, image) in (&fades, &mut images).join() {
                    if let UiImage::SolidColor(rgba) = image {
                        rgba[3] += time.delta_seconds() / DEFAULT_TIMER;
                    }
                }
            }
            self.fade_in_timer -= time.delta_seconds();
        }
    }
}
