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

struct CacheNode {
    name: String,
    position: (f64, f64)
}

struct CacheConnection {
    from: (f64, f64),
    to: (f64, f64)
}

struct Cache {
    nodes: Vec<CacheNode>,
    connections: Vec<CacheConnection>
}

struct App {
    db: IdeaDatabase,
    cache: Option<Cache>
}

impl App {
    fn update_cache(&mut self) {
        let nodes = self.db.get_thing_info().unwrap();
        let connections = self.db.get_connection_info().unwrap();

        self.cache = Some(Cache {
            nodes: nodes
                .iter()
                .map(|ref node| CacheNode {
                    name: node.thing.name.clone(),
                    position: (node.thing.x as f64, node.thing.y as f64)
                })
                .collect(),

            connections: connections
                .iter()
                .map(|connection| CacheConnection {
                    from: (connection.from.0 as f64, connection.from.1 as f64),
                    to: (connection.to.0 as f64, connection.to.1 as f64)
                })
                .collect()
        });
    }
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, glyphs: &mut GlyphCache) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        if self.cache.is_none() {
            self.update_cache();
        }

        gl.draw(
            args.viewport(),
            |context, gl| {
                clear(GREEN, gl);

                if let Some(ref cache) = self.cache {
                    for connection in &cache.connections {
                        line(BLACK, 2.0, [connection.from.0, connection.from.1, connection.to.0, connection.to.1], context.transform, gl);
                    }

                    for node in &cache.nodes {
                        let transform = context.transform.trans(node.position.0, node.position.1);
                        ellipse(RED, [-20.0, -20.0, 40.0, 40.0], transform, gl);

                        let width = glyphs.width(16, &node.name).unwrap();
                        let transform = transform.trans(-width / 2.0, -24.0);
                        text(BLACK, 16, &node.name, glyphs, transform, gl).unwrap();
                    }
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
        db,
        cache: None
    };

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(ref e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl, &mut glyphs);
        }
    }
}
