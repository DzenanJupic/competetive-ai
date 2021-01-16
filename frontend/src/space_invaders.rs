use std::rc::Rc;

use seed::{*, prelude::*};
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent};

use space_invaders::{GameObj, Instruction, PlayField, Position};
use space_invaders::alien::{Alien, Aliens, AlienType};
use space_invaders::bullet::Bullet;
use space_invaders::bunker::Bunkers;

use crate::GMsg;

// About 30 FPS 
const STEP_MILLI_SECONDS: u32 = 34;

thread_local! {
    static RED: Rc<JsValue> = Rc::new(JsValue::from_str("#FF6000"));
    static GREEN: Rc<JsValue> = Rc::new(JsValue::from_str("#1BBE81"));
    static WHITE: Rc<JsValue> = Rc::new(JsValue::from_str("#fff"));
}


pub(crate) struct Model {
    play_field: PlayField,
    canvas: ElRef<HtmlCanvasElement>,
    keyboard_listener: StreamHandle,
    instruction: Instruction,
    shoot: bool,
    game_state: GameState,
}

pub(crate) enum Msg {
    StartGame,
    PauseGame,
    ResetGame,
    KeyBoardEvent(KeyboardEvent),
    Render,
}

pub(crate) enum GameState {
    Running,
    Paused,
    None,
}

impl Model {
    pub(crate) fn new(orders: &mut impl Orders<GMsg>) -> Self {
        let keyboard_listener = orders.stream_with_handle(
            streams::window_event(Ev::KeyDown, |ev: Event| {
                GMsg::SpaceInvaders(Msg::KeyBoardEvent(ev.unchecked_into()))
            })
        );

        let model = Self {
            play_field: PlayField::new(),
            canvas: ElRef::new(),
            keyboard_listener,
            instruction: Instruction::None,
            shoot: false,
            game_state: GameState::Running,
        };
        model.schedule_step(orders);
        model
    }

    pub(crate) fn update(&mut self, msg: Msg, orders: &mut impl Orders<GMsg>) {
        match msg {
            Msg::StartGame => {
                if let GameState::None = self.game_state {
                    self.game_state = GameState::Running;
                    self.step();
                }
            }
            Msg::PauseGame => {
                match self.game_state {
                    GameState::Running => self.game_state = GameState::Paused,
                    GameState::Paused => self.game_state = GameState::Running,
                    GameState::None => {}
                }
            }
            Msg::ResetGame => {
                self.play_field = PlayField::new();
                self.game_state = GameState::None;
            }
            Msg::KeyBoardEvent(ev) => {
                match &*ev.key() {
                    "ArrowLeft" => self.instruction = Instruction::MoveLeft,
                    "ArrowRight" => self.instruction = Instruction::MoveRight,
                    " " => self.shoot = true,
                    _ => {}
                }
            }
            Msg::Render => {
                self.step();
                self.draw_play_field();
                self.schedule_step(orders);
            }
        }
    }

    pub(crate) fn view(&self) -> Node<Msg> {
        canvas![
            el_ref(&self.canvas),
            C!["d-block", "position-absolute", "w-100", "h-100"],
            style! {
                St::Top => "0",
                St::Left => "0",
            },
            attrs! {
                At::Width => PlayField::WIDTH,
                At::Height => PlayField::HEIGHT,
            },
        ]
    }

    fn step(&mut self) {
        log::trace!("step: {:?} | shoot: {}", self.instruction, self.shoot);
        let survived = self.play_field.step(self.instruction, self.shoot);
        log::trace!("player survived: {}", survived);
        self.instruction = Instruction::None;
        self.shoot = false;
    }

    fn schedule_step(&self, orders: &mut impl Orders<GMsg>) {
        if let GameState::Running = self.game_state {
            orders
                .perform_cmd(async {
                    cmds::timeout(STEP_MILLI_SECONDS, || {}).await;
                    GMsg::SpaceInvaders(Msg::Render)
                });
        }
    }

    fn draw_play_field(&self) {
        let canvas = self.canvas.get().expect("could not get canvas");
        let ctx = seed::canvas_context_2d(&canvas);

        Self::clear_canvas(&canvas, &ctx);
        Self::draw_bullets(&ctx, self.play_field.bullets());
        Self::draw_aliens(&ctx, self.play_field.aliens());
        Self::draw_bunkers(&ctx, self.play_field.bunkers());
        Self::draw_cannon(&ctx, self.play_field.cannon().position());
    }

    fn clear_canvas(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
    }

    fn draw_aliens(ctx: &CanvasRenderingContext2d, aliens: &Aliens) {
        aliens
            .iter()
            .map(|col| col.iter())
            .flatten()
            .for_each(|alien| {
                if let Some(alien) = alien {
                    Self::draw_alien(ctx, alien);
                }
            })
    }

    fn draw_alien(ctx: &CanvasRenderingContext2d, alien: &Alien) {
        WHITE.with(|white| {
            ctx.set_fill_style(white)
        });

        match alien.alien_type() {
            AlienType::Mystery => Self::draw_alien_mystery(ctx, alien.position()),
            AlienType::Hard => Self::draw_alien_hard(ctx, alien.position()),
            AlienType::Medium => Self::draw_alien_medium(ctx, alien.position()),
            AlienType::Easy => Self::draw_alien_easy(ctx, alien.position()),
        }
    }

    fn draw_alien_hard(ctx: &CanvasRenderingContext2d, Position { mut x, y }: Position) {
        // hard aliens are four pixels smaller then easy aliens
        // space invaders centers them, so we do the same
        x += 2;

        ctx.fill_rect(x as f64 + 3., y as f64 + 0., 2., 1.);
        ctx.fill_rect(x as f64 + 2., y as f64 + 1., 4., 1.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 2., 6., 1.);
        ctx.fill_rect(x as f64 + 0., y as f64 + 3., 2., 2.);
        ctx.fill_rect(x as f64 + 3., y as f64 + 3., 2., 4.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 3., 2., 2.);
        ctx.fill_rect(x as f64 + 2., y as f64 + 4., 4., 1.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 5., 1., 1.);
        ctx.fill_rect(x as f64 + 0., y as f64 + 6., 1., 1.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 7., 1., 1.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 5., 1., 1.);
        ctx.fill_rect(x as f64 + 7., y as f64 + 6., 1., 1.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 7., 1., 1.);
    }

    fn draw_alien_medium(ctx: &CanvasRenderingContext2d, Position { mut x, y }: Position) {
        // medium aliens are one pixel smaller then easy aliens
        // space invaders aligns them to the right, so we do the same
        x += 1;

        ctx.fill_rect(x as f64 + 2., y as f64 + 0., 1., 1.);
        ctx.fill_rect(x as f64 + 3., y as f64 + 1., 1., 1.);
        ctx.fill_rect(x as f64 + 8., y as f64 + 0., 1., 1.);
        ctx.fill_rect(x as f64 + 7., y as f64 + 1., 1., 1.);
        ctx.fill_rect(x as f64 + 2., y as f64 + 2., 7., 1.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 3., 2., 1.);
        ctx.fill_rect(x as f64 + 4., y as f64 + 3., 3., 1.);
        ctx.fill_rect(x as f64 + 8., y as f64 + 3., 2., 1.);
        ctx.fill_rect(x as f64 + 0., y as f64 + 4., 11., 2.);
        ctx.fill_rect(x as f64 + 0., y as f64 + 6., 1., 1.);
        ctx.fill_rect(x as f64 + 2., y as f64 + 6., 1., 1.);
        ctx.fill_rect(x as f64 + 8., y as f64 + 6., 1., 1.);
        ctx.fill_rect(x as f64 + 10., y as f64 + 6., 1., 1.);
        ctx.fill_rect(x as f64 + 3., y as f64 + 7., 2., 1.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 7., 2., 1.);
    }

    fn draw_alien_easy(ctx: &CanvasRenderingContext2d, Position { x, y }: Position) {
        ctx.fill_rect(x as f64 + 4., y as f64 + 0., 4., 1.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 1., 10., 2.);
        ctx.fill_rect(x as f64 + 0., y as f64 + 2., 3., 3.);
        ctx.fill_rect(x as f64 + 5., y as f64 + 3., 2., 1.);
        ctx.fill_rect(x as f64 + 9., y as f64 + 2., 3., 3.);
        ctx.fill_rect(x as f64 + 3., y as f64 + 4., 6., 1.);
        ctx.fill_rect(x as f64 + 2., y as f64 + 5., 3., 1.);
        ctx.fill_rect(x as f64 + 7., y as f64 + 5., 3., 1.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 6., 2., 1.);
        ctx.fill_rect(x as f64 + 5., y as f64 + 6., 2., 1.);
        ctx.fill_rect(x as f64 + 9., y as f64 + 6., 2., 1.);
        ctx.fill_rect(x as f64 + 2., y as f64 + 7., 2., 1.);
        ctx.fill_rect(x as f64 + 8., y as f64 + 7., 2., 1.);
    }

    fn draw_alien_mystery(ctx: &CanvasRenderingContext2d, Position { x, y }: Position) {
        RED.with(|red| {
            ctx.set_fill_style(red)
        });

        todo!()
    }

    fn draw_bunkers(ctx: &CanvasRenderingContext2d, bunkers: &Bunkers) {
        GREEN.with(|green| {
            ctx.set_fill_style(green)
        });

        bunkers
            .iter()
            .for_each(|bunker| {
                if let Some(bunker) = bunker {
                    Self::draw_bunker(ctx, bunker.position())
                }
            })
    }

    fn draw_bunker(ctx: &CanvasRenderingContext2d, Position { x, y }: Position) {
        ctx.fill_rect(x as f64 + 3., y as f64 + 0., 18., 1.);
        ctx.fill_rect(x as f64 + 2., y as f64 + 1., 20., 1.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 2., 22., 1.);
        ctx.fill_rect(x as f64 + 0., y as f64 + 3., 6., 15.);
        ctx.fill_rect(x as f64 + 18., y as f64 + 3., 6., 15.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 3., 12., 5.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 8., 4., 1.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 9., 3., 1.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 10., 2., 1.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 11., 1., 1.);
        ctx.fill_rect(x as f64 + 14., y as f64 + 8., 4., 1.);
        ctx.fill_rect(x as f64 + 15., y as f64 + 9., 3., 1.);
        ctx.fill_rect(x as f64 + 16., y as f64 + 10., 2., 1.);
        ctx.fill_rect(x as f64 + 17., y as f64 + 11., 1., 1.);
    }

    fn draw_cannon(ctx: &CanvasRenderingContext2d, Position { x, y }: Position) {
        GREEN.with(|white| {
            ctx.set_fill_style(white)
        });

        ctx.fill_rect(x as f64 + 7., y as f64 + 0., 1., 1.);
        ctx.fill_rect(x as f64 + 6., y as f64 + 1., 3., 2.);
        ctx.fill_rect(x as f64 + 1., y as f64 + 3., 13., 1.);
        ctx.fill_rect(x as f64 + 0., y as f64 + 4., 15., 4.);
    }

    fn draw_bullets(ctx: &CanvasRenderingContext2d, bullets: &Vec<Bullet>) {
        WHITE.with(|white| {
            ctx.set_fill_style(white)
        });

        bullets
            .iter()
            .for_each(|bullet| Self::draw_bullet(ctx, bullet.position()));
    }

    fn draw_bullet(ctx: &CanvasRenderingContext2d, Position { x, y }: Position) {
        ctx.fill_rect(x as f64, y as f64, 1., 3.);
    }
}




