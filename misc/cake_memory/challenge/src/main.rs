#![windows_subsystem = "windows"]

use ggez::audio;
use ggez::audio::SoundSource;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, Mesh, MeshBuilder, TextAlign, TextLayout};
use ggez::input::mouse;
use ggez::{Context, ContextBuilder, GameResult};
use glam::Vec2;
use obfstr::obfstr;
use std::collections::HashMap;
use std::path;
use std::vec::Vec;
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SoundName {
    VoiceBlue, VoiceBurgundy, VoiceCake, VoiceCoquelicot,
    VoiceE, VoiceFlag, VoiceG, VoiceGreen, VoiceHeart,
    VoiceJ, VoiceKitten, VoiceLambda, VoiceOmega,
    VoiceOrange, VoicePi, VoiceQuestion, VoiceRed,
    VoiceSmiley, VoiceTangerine, VoiceTheta,
    VoiceTurquoise, VoiceViolet, VoiceWhite, VoiceYellow,
}

static SOUNDS: [SoundName; 24] = [
    SoundName::VoiceBlue, SoundName::VoiceRed,
    SoundName::VoiceYellow, SoundName::VoiceGreen,
    SoundName::VoiceViolet, SoundName::VoiceOrange,
    SoundName::VoiceWhite, SoundName::VoiceJ,
    SoundName::VoiceHeart, SoundName::VoiceQuestion,
    SoundName::VoicePi, SoundName::VoiceSmiley,
    SoundName::VoiceOmega, SoundName::VoiceTurquoise,
    SoundName::VoiceTheta, SoundName::VoiceG,
    SoundName::VoiceKitten, SoundName::VoiceTangerine,
    SoundName::VoiceCake, SoundName::VoiceLambda,
    SoundName::VoiceBurgundy, SoundName::VoiceE,
    SoundName::VoiceCoquelicot, SoundName::VoiceFlag
];

struct MusicalMemory {
    beep: Vec<(audio::Source, Mesh)>,
    misc: Vec<audio::Source>,
    sound: HashMap<SoundName, (audio::Source, Mesh, Mesh, String)>,
    round_clear: bool, // 小ラウンドをクリアしたか
    round_wrong: bool, // ゲームオーバー
    round_num: i32, // 現在のラウンド

    mem_clear: bool, // 小回答正解
    mem_count: usize, // 現在の回答箇所
    mem_count_: usize, // チート対策
    mem_round_num: usize, // 現在の小ラウンド
    mem_round_num_: usize, // チート対策
    mem_size: Vec<usize>, // 各ラウンドで何色出すかリスト
    mem_order: Vec<usize>, // 色リスト
    mem_sound: Vec<SoundName>, // 音名リスト

    show_count: usize, // 現在の表示箇所
    show_timer: u128, // 経過時間
    show_interval: u128, // 表示間隔

    timer: u128, // 残り時間
}

impl Default for MusicalMemory {
    fn default() -> MusicalMemory {
        MusicalMemory {
            beep: Vec::new(),
            misc: Vec::new(),
            sound: HashMap::new(),
            round_clear: true,
            round_wrong: false,
            round_num: 0,
            mem_clear: true,
            mem_count: 0xffff,
            mem_count_: 0,
            mem_round_num: 0,
            mem_round_num_: 1,
            mem_size: vec![],
            mem_order: vec![],
            mem_sound: vec![],
            show_count: 0,
            show_timer: 0,
            show_interval: 0,
            timer: 0
        }
    }
}

impl MusicalMemory {
    fn drop_flag(&mut self) -> String {
        let key = obfstr!("bunzo").to_string().into_bytes();
        let flag: Vec<u8> = vec![
            120, 239, 96, 134, 201, 74, 183, 101, 178, 169, 227, 171, 1, 250,
            125, 131, 130, 49, 146, 22, 131, 117, 153, 49, 106, 130, 65, 20,
            77, 143, 190, 198, 134, 28, 68
        ];

        let mut state: [u8; 256] = [0; 256];
        for (i, x) in state.iter_mut().enumerate() {
            *x = i as u8;
        }

        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(state[i]).wrapping_add(key[i % key.len()]);
            state.swap(i, j as usize);
        }

        let mut out: Vec<u8> = vec![];
        let (mut i, mut j): (u8, u8) = (0, 0);
        for c in flag.iter() {
            i = i.wrapping_add(1);
            j = j.wrapping_add(state[i as usize]);
            state.swap(i as usize, j as usize);
            out.push((self.round_num as u8) ^ c ^ state[
                (state[i as usize]).wrapping_add(state[j as usize]) as usize
            ]);
        }

        String::from_utf8(out).expect(obfstr!("Cheat detected!"))
    }
}

impl event::EventHandler for MusicalMemory {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if ctx.quit_requested {
            ctx.continuing = false;
        }

        if !self.round_clear && self.mem_round_num_ != self.mem_round_num * 77 + 1 {
            ctx.continuing = false;
            panic!("{}", obfstr!("Cheat detected!"));
        }

        if !self.mem_clear && self.mem_count_ != self.mem_count * 77 + 1 {
            ctx.continuing = false;
            panic!("{}", obfstr!("Cheat detected!"));
        }

        if self.mem_clear || self.round_wrong {
            return Ok(());
        }

        // Next round or show flag
        if self.round_clear {
            if self.mem_round_num_ != self.mem_size.len() * 77 + 1 {
                ctx.continuing = false;
                panic!("{}", obfstr!("Cheat detected!"))
            } else {
                self.mem_round_num_ = 1;
            }
            self.round_num = self.round_num * 2 + 1;
            self.mem_count_ = 1;
            self.mem_count = 0;

            self.mem_size = match self.round_num {
                1 => vec![1, 2, 3, 4],
                3 => vec![1, 2, 3, 4, 5, 6],
                7 => vec![1, 2, 3, 4, 5, 6, 7, 8],
                15 => vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                31 => vec![100],
                _ => panic!("{}", obfstr!("Cheat detected!"))
            };

            self.mem_sound = match self.round_num {
                1 => (&SOUNDS[0..4]).to_vec(),
                3 => (&SOUNDS[0..5]).to_vec(),
                7 => (&SOUNDS[0..7]).to_vec(),
                15 => (&SOUNDS[0..12]).to_vec(),
                31 => SOUNDS.to_vec(),
                _ => panic!("{}", obfstr!("Cheat detected!"))
            };

            self.show_interval = match self.round_num {
                1 => 800,
                31 => 300,
                _ => 600
            };

            let mut rng = rand::thread_rng();
            self.mem_order = (0..self.mem_size[self.mem_size.len()-1])
                .map(|_| rng.gen_range(0..self.mem_sound.len()))
                .collect();

            self.show_count = 0;
            self.show_timer = 0;
            self.round_clear = false;

            self.timer = match self.round_num {
                1 => 10 * 1000,
                3 => 15 * 1000,
                7 => 20 * 1000,
                15 => 30 * 1000,
                31 => 10 * 1000,
                _ => panic!("{}", obfstr!("Cheat detected!"))
            };

            return Ok(());
        }

        // TODO: remove me
        if ctx.mouse.button_just_pressed(mouse::MouseButton::Right) {
            self.mem_count += 1;
        }

        // Button check
        if ctx.mouse.button_just_pressed(mouse::MouseButton::Left)
            && self.show_count == self.mem_size[self.mem_round_num] {
            let mpos = ctx.mouse.position();
            for (i, snd) in self.mem_sound.iter().enumerate() {
                let pos = pos_by(self.mem_sound.len(), i);

                if pos.x <= mpos.x && mpos.x <= pos.x + 58.0
                    && pos.y <= mpos.y && mpos.y <= pos.y + 58.0
                {
                    if self.mem_sound[self.mem_order[self.mem_count]] == *snd {
                        // Correct
                        self.misc[5].play(ctx)?;
                        self.mem_count += 1;
                        self.mem_count_ += 77;
                        break;
                    } else {
                        // Wrong
                        self.beep[0].0.play(ctx)?;
                        self.round_wrong = true;
                        break;
                    }
                }
            }
        }

        // Sub-round check
        if self.mem_count >= self.mem_size[self.mem_round_num] {
            self.beep[1].0.set_volume(0.5);
            self.beep[1].0.play(ctx)?;
            self.show_count = 0;
            self.show_timer = 0;
            self.mem_count_ = 1;
            self.mem_count = 0;
            self.mem_round_num += 1;
            self.mem_round_num_ += 77;
            self.mem_clear = true;

            self.timer = match self.round_num {
                1 => 10 * 1000,
                3 => 15 * 1000,
                7 => 20 * 1000,
                15 => 30 * 1000,
                31 => 10 * 1000,
                _ => panic!("{}", obfstr!("Cheat detected!"))
            };
        }

        // Round check
        if self.mem_round_num >= self.mem_size.len() {
            self.mem_count = 0xffff;
            self.mem_count_ = 0;
            self.mem_round_num = 0;
            self.round_clear = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let current;
        let mut canvas = graphics::Canvas::from_frame(
            ctx, Color::from((10, 10, 10))
        );

        if self.round_wrong {
            // Game over screen
            canvas.draw(&self.beep[0].1, graphics::DrawParam::from([0., 0.]));
            canvas.draw(
                graphics::Text::new(obfstr!("GAME OVER"))
                    .set_scale(128.)
                    .set_bounds(Vec2::new(800.0, 600.0),
                                TextLayout::Wrap {
                                    h_align: TextAlign::Middle,
                                    v_align: TextAlign::Middle,
                                }),
                graphics::DrawParam::from([0., 0.])
                    .color(Color::from((172, 35, 47))),
            );

            return canvas.finish(ctx);
        }

        if self.mem_clear {
            if self.round_num == 31 {
                // Game clear
                canvas.draw(&self.beep[1].1, graphics::DrawParam::from([0., 0.]));
                canvas.draw(
                    graphics::Text::new(obfstr!("YOU WON"))
                        .set_scale(128.)
                        .set_bounds(Vec2::new(800.0, 600.0),
                                    TextLayout::Wrap {
                                        h_align: TextAlign::Middle,
                                        v_align: TextAlign::Middle,
                                    }),
                    graphics::DrawParam::from([0., 0.])
                        .color(Color::from((16, 16, 16))),
                );
                canvas.draw(
                    graphics::Text::new(self.drop_flag())
                        .set_scale(32.)
                        .set_bounds(Vec2::new(800.0, 120.0),
                                    TextLayout::Wrap {
                                        h_align: TextAlign::Middle,
                                        v_align: TextAlign::Middle,
                                    }),
                    graphics::DrawParam::from([0., 600.])
                        .color(Color::from((240, 240, 240)))
                );
                return canvas.finish(ctx);
            }
            
            // Round screen
            self.show_timer += ctx.time.delta().as_millis();

            if self.mem_round_num == 0 {
                let rnd = match self.round_num {
                    0 => 0, 1 => 1, 3 => 2, 7 => 3, 15 => 4,
                    _ => panic!("{}", obfstr!("Cheat detected!"))
                };
                let (a, b) = if rnd == 0 { (0, 2000) } else { (800, 2800) };

                if self.show_timer >= a && self.mem_count == 0xffff {
                    // Round voice
                    self.misc[rnd].play(ctx)?;
                    self.mem_count_ = 1;
                    self.mem_count = 0;

                } else if self.show_timer >= b {
                    // Start round
                    self.show_timer = 0;
                    self.mem_clear = false;
                }

                if self.show_timer >= a {
                    // Round screen
                    canvas.draw(&self.beep[1].1, graphics::DrawParam::from([0., 0.]));
                    canvas.draw(
                        graphics::Text::new(format!("ROUND-{}", rnd + 1))
                            .set_scale(128.)
                            .set_bounds(Vec2::new(800.0, 600.0),
                                        TextLayout::Wrap {
                                            h_align: TextAlign::Middle,
                                            v_align: TextAlign::Middle,
                                        }),
                        graphics::DrawParam::from([0., 0.])
                            .color(Color::from((16, 16, 16))),
                    );
                } else {
                    return Ok(());
                }

                return canvas.finish(ctx);
            } else {
                if self.show_timer >= 1000 {
                    // Start round
                    self.show_timer = 0;
                    self.mem_clear = false;
                }
                return Ok(());
            }
        }

        if self.round_clear {
            return Ok(());
        }

        if self.show_count < self.mem_size[self.mem_round_num] {
            current = self.sound.get_mut(&self.mem_sound[
                self.mem_order[self.show_count]
            ]);
        } else {
            current = self.sound.get_mut(&self.mem_sound[
                self.mem_order[self.mem_size[self.mem_round_num] - 1]
            ]);
        }

        let (sound, mesh, _, text) = current.unwrap();

        /* Sound */
        if self.show_count < self.mem_size[self.mem_round_num] {
            if self.show_timer == 0 {
                // Play sound
                sound.play(ctx)?;
            }

            // Switch timer
            self.show_timer += ctx.time.delta().as_millis();

            if self.show_timer >= self.show_interval {
                self.show_count += 1;
                self.show_timer = 0;
            }
        }

        /* Draw */
        // Draw screen
        canvas.draw(
            mesh,
            graphics::DrawParam::from([0., 0.])
        );

        // Draw text
        canvas.draw(
            graphics::Text::new(&*text)
                .set_scale(128.)
                .set_bounds(Vec2::new(800.0, 600.0),
                            TextLayout::Wrap {
                                h_align: TextAlign::Middle,
                                v_align: TextAlign::Middle,
                            }),
            graphics::DrawParam::from([0., 0.])
                .color(Color::from((10, 10, 10))),
        );

        if self.show_count == self.mem_size[self.mem_round_num] {
            for (i, snd) in self.mem_sound.iter().enumerate() {
                let pos = pos_by(self.mem_sound.len(), i);
                let (_, _, button, text) = self.sound.get_mut(snd).unwrap();
                // Draw button
                canvas.draw(
                    button,
                    graphics::DrawParam::from(pos)
                );
                // Draw button text
                match text.as_str() {
                    "CAKE" | "E" | "FLAG" | "G" | "<3" | "J" | "ΦωΦ"
                        | "λ" | "ω" | "π" | "?" | ":)" | "θ" => (
                            canvas.draw(
                                graphics::Text::new(&*text)
                                    .set_scale(26.)
                                    .set_bounds(Vec2::new(58.0, 58.0),
                                                TextLayout::Wrap {
                                                    h_align: TextAlign::Middle,
                                                    v_align: TextAlign::Middle,
                                                }),
                                graphics::DrawParam::from(pos)
                                    .color(Color::from((10, 10, 10))),
                            )
                        ),
                    _ => ()
                };
            }

            // Draw timer
            self.timer = self.timer.saturating_sub(ctx.time.delta().as_millis());
            let v = self.timer / 1000;
            canvas.draw(
                graphics::Text::new(format!("LIMIT: {:02}", v))
                    .set_scale(32.)
                    .set_bounds(Vec2::new(800.0, 0.0),
                                TextLayout::Wrap {
                                    h_align: TextAlign::End,
                                    v_align: TextAlign::Begin,
                                }),
                graphics::DrawParam::from(Vec2::new(0., 0.))
                    .color(Color::from((255, 255, 255))),
            );

            if v == 0 {
                self.beep[0].0.play(ctx)?;
                self.round_wrong = true;
            }
        }

        canvas.finish(ctx)
    }

}

impl MusicalMemory {
    /**
     * Load game resource
     */
    pub fn new(ctx: &mut Context) -> GameResult<MusicalMemory> {
        let beep = vec![
            (audio::Source::new(ctx, obfstr!("/snd/beep/wrong.wav"))?,
             build_mesh(ctx, Color::from((64, 64, 64)))),
            (audio::Source::new(ctx, obfstr!("/snd/beep/correct.wav"))?,
             build_mesh(ctx, Color::from((240, 240, 240))))
        ];

        let misc = vec![
            audio::Source::new(ctx, obfstr!("/snd/voice/r1.wav"))?,
            audio::Source::new(ctx, obfstr!("/snd/voice/r2.wav"))?,
            audio::Source::new(ctx, obfstr!("/snd/voice/r3.wav"))?,
            audio::Source::new(ctx, obfstr!("/snd/voice/r4.wav"))?,
            audio::Source::new(ctx, obfstr!("/snd/voice/r5.wav"))?,
            audio::Source::new(ctx, obfstr!("/snd/beep/click.wav"))?
        ];

        let mut sound = HashMap::new();
        sound.insert(SoundName::VoiceBlue,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/blue.wav"))?,
                      build_mesh(ctx, Color::from((0, 0, 255))),
                      build_button(ctx, Color::from((0, 0, 255))),
                      obfstr!("BLUE").to_string()));
        sound.insert(SoundName::VoiceBurgundy,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/burgundy.wav"))?,
                      build_mesh(ctx, Color::from((128, 0, 32))),
                      build_button(ctx, Color::from((128, 0, 32))),
                      obfstr!("BURGUNDY").to_string()));
        sound.insert(SoundName::VoiceCake,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/cake.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("CAKE").to_string()));
        sound.insert(SoundName::VoiceCoquelicot,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/coquelicot.wav"))?,
                      build_mesh(ctx, Color::from((255, 56, 0))),
                      build_button(ctx, Color::from((255, 56, 0))),
                      obfstr!("COQUELICOT").to_string()));
        sound.insert(SoundName::VoiceE,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/e.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("E").to_string()));
        sound.insert(SoundName::VoiceFlag,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/flag.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("FLAG").to_string()));
        sound.insert(SoundName::VoiceG,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/g.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("G").to_string()));
        sound.insert(SoundName::VoiceGreen,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/green.wav"))?,
                      build_mesh(ctx, Color::from((0, 255, 0))),
                      build_button(ctx, Color::from((0, 255, 0))),
                      obfstr!("GREEN").to_string()));
        sound.insert(SoundName::VoiceHeart,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/heart.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("<3").to_string()));
        sound.insert(SoundName::VoiceJ,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/j.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("J").to_string()));
        sound.insert(SoundName::VoiceKitten,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/kitten.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("ΦωΦ").to_string()));
        sound.insert(SoundName::VoiceLambda,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/lambda.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("λ").to_string()));
        sound.insert(SoundName::VoiceOmega,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/omega.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("Ω").to_string()));
        sound.insert(SoundName::VoiceOrange,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/orange.wav"))?,
                      build_mesh(ctx, Color::from((255, 165, 0))),
                      build_button(ctx, Color::from((255, 165, 0))),
                      obfstr!("ORANGE").to_string()));
        sound.insert(SoundName::VoicePi,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/pi.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("π").to_string()));
        sound.insert(SoundName::VoiceQuestion,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/question.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("?").to_string()));
        sound.insert(SoundName::VoiceRed,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/red.wav"))?,
                      build_mesh(ctx, Color::from((255, 0, 0))),
                      build_button(ctx, Color::from((255, 0, 0))),
                      obfstr!("RED").to_string()));
        sound.insert(SoundName::VoiceSmiley,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/smiley.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!(":)").to_string()));
        sound.insert(SoundName::VoiceTangerine,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/tangerine.wav"))?,
                      build_mesh(ctx, Color::from((242, 133, 0))),
                      build_button(ctx, Color::from((242, 133, 0))),
                      obfstr!("TANGERINE").to_string()));
        sound.insert(SoundName::VoiceTheta,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/theta.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("θ").to_string()));
        sound.insert(SoundName::VoiceTurquoise,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/turquoise.wav"))?,
                      build_mesh(ctx, Color::from((48, 213, 200))),
                      build_button(ctx, Color::from((48, 213, 200))),
                      obfstr!("TURQUOISE").to_string()));
        sound.insert(SoundName::VoiceViolet,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/violet.wav"))?,
                      build_mesh(ctx, Color::from((238, 130, 238))),
                      build_button(ctx, Color::from((238, 130, 238))),
                      obfstr!("VIOLET").to_string()));
        sound.insert(SoundName::VoiceWhite,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/white.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 255))),
                      build_button(ctx, Color::from((255, 255, 255))),
                      obfstr!("WHITE").to_string()));
        sound.insert(SoundName::VoiceYellow,
                     (audio::Source::new(ctx, obfstr!("/snd/voice/yellow.wav"))?,
                      build_mesh(ctx, Color::from((255, 255, 0))),
                      build_button(ctx, Color::from((255, 255, 0))),
                      obfstr!("YELLOW").to_string()));

        Ok(MusicalMemory { beep, misc, sound, ..Default::default() })
    }
}

fn build_mesh(ctx: &mut Context, color: Color) -> graphics::Mesh {
    let mb = &mut MeshBuilder::new();
    for i in 0..5 {
        for j in 0..5 {
            let _ = mb.rounded_rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(5.0 + (i*160) as f32,
                                    5.0 + (j*118) as f32,
                                    150.0, 112.0),
                8.0,
                color
            );
        }
    }

    let _ = mb.rectangle(
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, 800.0, 600.0),
        Color::from([0., 0., 0., 0.3])
    );
    for i in 0..37 {
        let _ = mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, (16*i) as f32, 800.0, 8.0),
            Color::from([0., 0., 0., 0.4])
        );
    }

    Mesh::from_data(ctx, mb.build())
}

fn build_button(ctx: &mut Context, color: Color) -> graphics::Mesh {
    let mb = &mut MeshBuilder::new();
    let _ = mb.rectangle(
        graphics::DrawMode::fill(),
        graphics::Rect::new(0., 0., 58., 58.),
        color
    );
    let _ = mb.rectangle(
        graphics::DrawMode::stroke(2.),
        graphics::Rect::new(0., 0., 58., 58.),
        Color::from((128, 128, 128))
    );

    Mesh::from_data(ctx, mb.build())
}

fn pos_by(len: usize, i: usize) -> Vec2 {
    match len {
        4 => Vec2::new(130.0 + (i*160) as f32, 630.0),
        5 => Vec2::new(100.0 + (i*130) as f32, 630.0),
        7 => Vec2::new(70.0 + (i*100) as f32, 630.0),
        12 => Vec2::new(14.0 + (i*65) as f32, 630.0),
        24 => Vec2::new(14.0 + ((i%12)*65) as f32, 596.0 + ((i/12)*62) as f32),
        _ => panic!("{}", obfstr!("Cheat detected!"))
    }
}

fn main() {
    /* Add resource directory */
    let resource_dir =  path::PathBuf::from("./resources");
    let cb = ContextBuilder::new("musical-memory", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(
            conf::WindowSetup {
                title: "Cake Memory".to_string(),
                samples: conf::NumSamples::One,
                icon: "".to_owned(),
                vsync: true,
                srgb: true
            }
        )
        .window_mode(
            conf::WindowMode {
                width: 800., height: 720.,
                maximized: false,
                fullscreen_type: conf::FullscreenType::Windowed,
                borderless: false,
                min_width: 1., max_width: 0.,
                min_height: 1., max_height: 0.,
                resizable: false,
                visible: true,
                transparent: false,
                resize_on_scale_factor_change: false
            }
        );

    /* Create context */
    let (mut ctx, event_loop) = cb.build()
        .expect("Your environment cannot run this program.");

    let state = MusicalMemory::new(&mut ctx).expect("test");
    event::run(ctx, event_loop, state);
}
