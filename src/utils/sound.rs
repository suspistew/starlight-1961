use amethyst::audio::{SourceHandle, OggFormat, AudioSink, Source};
use std::{iter::Cycle, vec::IntoIter};
use amethyst::assets::{Loader, AssetStorage};
use amethyst::core::ecs::{World, WorldExt};
use amethyst::audio::output::{Output, init_output};

const MENU_MUSIC: &'static [&'static str] = &[
    "audio/menu_music.ogg"
];

const EXPLOSION: &str = "audio/explosion.ogg";
const FIRE: &str = "audio/fire.ogg";
const HIT: &str = "audio/hit.ogg";
const LAND: &str = "audio/land.ogg";
const BONUS: &str = "audio/bonus.ogg";
const AIR: &str = "audio/air.ogg";

pub struct Sounds {
    pub explosion: SourceHandle,
    pub fire: SourceHandle,
    pub hit: SourceHandle,
    pub air: SourceHandle,
    pub land: SourceHandle,
    pub bonus: SourceHandle,
    pub menu_music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialise_audio(world: &mut World) {
    init_output(world);
    let sounds = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.5);
        let menu_music = MENU_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, &file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let sounds = Sounds {
            explosion: load_audio_track(&loader, &world, EXPLOSION),
            fire: load_audio_track(&loader, &world, FIRE),
            hit: load_audio_track(&loader, &world, HIT),
            land: load_audio_track(&loader, &world, LAND),
            bonus: load_audio_track(&loader, &world, BONUS),
            air: load_audio_track(&loader, &world, AIR),
            menu_music };

        sounds
    };
    world.insert(sounds);
}

pub fn play_explosion(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.explosion) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_fire(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.fire) {
            output.play_once(sound, 0.2);
        }
    }
}

pub fn play_hit(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.hit) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_land(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.land) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_bonus(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bonus) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_air(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bonus) {
            output.play_once(sound, 1.0);
        }
    }
}