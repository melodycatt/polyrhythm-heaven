use std::f64::consts::PI;
use std::time::Duration;
use std::vec;
use rodio::{source::{Amplify, SineWave, TakeDuration}, OutputStream, OutputStreamHandle, Source};

use ggez::{
    conf::{WindowMode, WindowSetup}, 
    event::{self, EventHandler}, 
    graphics::{
        self, 
        Color, 
        Mesh, MeshBuilder, 
        Text
    }, 
    mint::{Point2, Vector2}, 
    Context, ContextBuilder, 
    GameResult
};

use augh::vectors::Vector;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(WindowMode::default().dimensions(1000.0, 1000.0).borderless(true))
        .window_setup(WindowSetup::default().vsync(false))
        .build()
        .expect("aieee, could not create ggez context!");


    let my_game = MyGame::new(&mut ctx, false, 5, 1.0);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    pos: Vec<Vector<f64>>,
    verts: Vec<Vec<Vector<f64>>>,
    indices: Vec<usize>,
    distances: Vec<f64>,

    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    freqs: Vec<TakeDuration<Amplify<SineWave>>>
}



impl MyGame {
    pub fn new(_ctx: &mut Context, sync: bool, n: i32, speed: f64) -> MyGame {
        let mut circumradius = 1.0;
        let mut verts: Vec<Vec<Vector<f64>>> = vec![vec![]];
        for i in 0..3 {
            verts[0].push(Vector::magnitude_angle(2.0 * PI / 3.0 * i as f64 + PI, circumradius));
        }
        let mut freqs: Vec<TakeDuration<Amplify<SineWave>>> = vec![];    
        let freq = 1600.0 / n as f32;
        freqs.push(SineWave::new(400.0).amplify(0.02).take_duration(Duration::from_millis(200)));
        for n in 4..=n+2 {
            circumradius *= ((PI / n as f64).tan().powi(2) + 1.0).sqrt();
            let mut poly_verts: Vec<Vector<f64>> = vec![];
            for i in 0..n {
                poly_verts.push(Vector::magnitude_angle(2.0 * PI / n as f64 * i as f64 + PI, circumradius));
            }    
            freqs.push(SineWave::new((n - 2) as f32 * freq + 400.0).amplify(0.02).take_duration(Duration::from_millis(200)));
            verts.push(poly_verts);
        }

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        MyGame {
            pos: verts.iter().map(|x| x[0]).collect(),
            indices: verts.iter().map(|_| 0).collect(),
            distances: verts.iter().map(|x| {
                x[0].distance(x[1]) * if sync { x.len() as f64 * speed } else { 6.5 * speed }
            }).collect(),
            verts,

            _stream,
            stream_handle,
            freqs
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for i in 0..self.pos.len() {
            let time = ctx.time.delta().as_secs_f64() / 4.0 * self.distances[i];
            let d = self.pos[i].distance(self.verts[i][self.indices[i]]);
            self.pos[i] = self.pos[i].move_towards(self.verts[i][self.indices[i]], time);
            if self.pos[i] == self.verts[i][self.indices[i]] { 
                self.indices[i] += 1; self.indices[i] %= i + 3;
                if time > d {
                    self.pos[i] = self.pos[i].move_towards(self.verts[i][self.indices[i]], time - d);
                }
                if self.indices[i] == 1 {
                    let s = self.freqs[i].clone();
                    let sink = rodio::Sink::try_new(&self.stream_handle).unwrap();
                    sink.append(s);
                    sink.detach();
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let mut mb = MeshBuilder::new();

        for i in 0..self.pos.len() {
            mb.polygon(graphics::DrawMode::stroke(0.1),&self.verts[i], Color::WHITE)?;
            let c = hsv::hsv_to_rgb(360.0 / (self.pos.len()) as f64 * i as f64, 1.0, 1.0);
            mb.circle(graphics::DrawMode::fill(),self.pos[i], 0.2, 0.005, Color::from_rgb(c.0, c.1, c.2))?;
        }
        let fps = Text::new(format!("{}", ctx.time.fps()));

        canvas.draw(&Mesh::from_data(ctx, mb.build()), graphics::DrawParam::default().dest(Point2 { x: 500.0, y: 500.0}).scale(Vector2 {x: 100.0, y: 100.0}));
        canvas.draw(&fps, graphics::DrawParam::default().dest(Point2 { x: 10.0, y: 10.0 }).color(Color::WHITE));

        canvas.finish(ctx)
    }
}