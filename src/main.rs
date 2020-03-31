use piston::{
    event_loop::{EventSettings, Events},
    input::{RenderArgs, RenderEvent},
    window::WindowSettings
};

use graphics::{
    character::CharacterCache,
    *
};

use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};

use find_folder::Search;

use idea_db::{types, IdeaDatabase};

struct App {
    gl: GlGraphics,
    db: IdeaDatabase
}

impl App {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let nodes = self.db.get_thing_info().unwrap();
        let connections = self.db.get_connection_info().unwrap();

        self.gl.draw(
            args.viewport(),
            |context, gl| {
                clear(GREEN, gl);

                for connection in connections {
                    let lhs = nodes.iter().find(|ref node| node.key == connection.connection.lhs).unwrap();
                    let rhs = nodes.iter().find(|ref node| node.key == connection.connection.rhs).unwrap();
                    line(BLACK, 2.0, [lhs.thing.x as f64, lhs.thing.y as f64, rhs.thing.x as f64, rhs.thing.y as f64], context.transform, gl);
                }

                for node in nodes {
                    let transform = context.transform.trans(node.thing.x as f64, node.thing.y as f64);
                    ellipse(RED, [-20.0, -20.0, 40.0, 40.0], transform, gl);

                    let width = glyphs.width(16, &node.thing.name).unwrap();
                    let transform = transform.trans(-width / 2.0, -24.0);
                    text(BLACK, 16, &node.thing.name, glyphs, transform, gl).unwrap();
                }
            });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Idea Graphs", (800, 600))
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Unable to open window");

    let assets_path = Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Unable to find assets folder");

    let mut glyphs = GlyphCache::new(assets_path.join("font.ttf"), (), TextureSettings::new()).unwrap();

    let db = {
        let path = assets_path.join("test.db");
        if let Ok(db) = IdeaDatabase::load(path) {
            db
        } else {
            let mut db = IdeaDatabase::new().unwrap();

            let kind = db
                .add_thing_kind(&types::ThingKind{
                    name: "test".to_string()
                })
                .unwrap();

            let n1 = db
                .add_thing(&types::Thing{
                    kind,
                    name: "Hello".to_string(),
                    x: 200,
                    y: 300
                })
                .unwrap();

            let n2 = db
                .add_thing(&types::Thing{
                    kind,
                    name: "World".to_string(),
                    x: 600,
                    y: 400
                })
                .unwrap();

            let n3 = db
                .add_thing(&types::Thing{
                    kind,
                    name: "Yay".to_string(),
                    x: 150,
                    y: 100
                })
                .unwrap();

            let conn_kind = db
                .add_connection_kind(&types::ConnectionKind {
                    name: "test".to_string(),
                    lhs: kind,
                    rhs: kind
                })
                .unwrap();

            db
                .add_connection(&types::Connection {
                    kind: conn_kind,
                    lhs: n1,
                    rhs: n2
                })
                .unwrap();

            db
                .add_connection(&types::Connection {
                    kind: conn_kind,
                    lhs: n1,
                    rhs: n3
                })
                .unwrap();

            db.save(assets_path.join("test.db")).unwrap();
            db
        }
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        db
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(ref e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut glyphs);
        }
    }
}
