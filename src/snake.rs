use amethyst::{
    core::{
        cgmath::Vector3,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component,VecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle,ScreenDimensions},
};
use rand;
use rand::Rng;



pub struct Snake {
    pub last_head_pos: Vector3<f32>,
    pub last_head_dir: SegmentDirection,
    pub food_available: bool,
    pub score: u64,
}
impl Snake {
    pub fn new(pos: Vector3<f32>,dir: SegmentDirection) -> Self {
        Snake {
            last_head_pos: pos,
            last_head_dir: dir,
            food_available: false,
            score: 0,
        }
    }
}

#[derive(PartialEq,Eq,Debug)]
pub enum SegmentType {
    Head,
    Body,
}

#[derive(Debug,Clone,Copy)]
pub enum SegmentDirection {
    Left,
    Right,
    Up,
    Down,
    Idle,
}

#[derive(Debug)]
pub struct Segment{
    pub t: SegmentType,
    pub direction: SegmentDirection,
    pub id: u64,
}
impl Segment {
    pub fn body(direction: SegmentDirection,id: u64) -> Self {
        Segment {
            t: SegmentType::Body,
            direction: direction,
            id: id,
        }
    }
}
impl Default for Segment {
    fn default() -> Self {
        Segment {
            t: SegmentType::Head,
            direction: SegmentDirection::Idle,
            id: 0,
        }
    }
}
impl Component for Segment {
    type Storage = VecStorage<Self>;
}



pub fn initialise_snake(world: &mut World,sheet_handle: SpriteSheetHandle){
    world.register::<Segment>();

    let snake_color_id = rand::thread_rng().gen_range(0,7);

    let snake_sprite = SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: snake_color_id,
        flip_horizontal: false,
        flip_vertical: false,
    };

    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        assert!(dimn.width() % 8.0 == 0.0, dimn.height() % 8.0 == 0.0);
        (dimn.width(), dimn.height())
    };
    
    let (x,y) = ((width / 16.0).round() * 8.0,(height / 16.0).round() * 8.0);

    world.add_resource(Snake::new(Vector3::new(x,y,0.0),SegmentDirection::Idle));

    world.create_entity()
                .with(snake_sprite)
                .with(GlobalTransform::default())
                .with(Transform::from(Vector3::new(x,y,0.0)))
                .with(Segment::default())
                .build();
}

