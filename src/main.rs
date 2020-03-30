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

use datagraph::{Graph, Index, Node, Connection};

#[derive(Copy, Clone)]
struct Position(f64, f64);

#[derive(Clone)]
struct Item {
    position: Position,
    label: String
}

#[derive(Clone)]
struct Colour([f32; 4]);

struct App {
    gl: GlGraphics,
    graph: Graph<Item, Colour>,
    nodes: Vec<Index<Node<Item, Colour>>>,
    connections: Vec<Index<Connection<Item, Colour>>>
}

impl App {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let connections = self.connections.iter().map(|index| self.graph.get(*index).unwrap().clone()).collect::<Vec<_>>();
        let nodes = self.nodes.iter().map(|index| (*index, self.graph.get(*index).unwrap().clone())).collect::<Vec<_>>();

        self.gl.draw(
            args.viewport(),
            |context, gl| {
                clear(GREEN, gl);

                for connection in connections {
                    let start = nodes.iter().find(|(index, _)| *index == connection.nodes()[0]).unwrap().1.data().position;
                    let end = nodes.iter().find(|(index, _)| *index == connection.nodes()[1]).unwrap().1.data().position;
                    let colour = connection.data();
                    line(colour.0, 2.0, [start.0, start.1, end.0, end.1], context.transform, gl);
                }

                for (_, node) in nodes {
                    let item = node.data();
                    let transform = context.transform.trans(item.position.0, item.position.1);
                    ellipse(RED, [-20.0, -20.0, 40.0, 40.0], transform, gl);

                    let width = glyphs.width(16, &item.label).unwrap();
                    let transform = transform.trans(-width / 2.0, -24.0);
                    text(BLACK, 16, &item.label, glyphs, transform, gl).unwrap();
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

    let mut graph = Graph::new();

    let nodes = vec![
        graph.add_node(Item {
            position: Position(200.0, 300.0),
            label: "Hello".to_string()
        }),
        graph.add_node(Item {
            position: Position(600.0, 400.0),
            label: "World".to_string()
        }),
        graph.add_node(Item {
            position: Position(150.0, 100.0),
            label: "Yay".to_string()
        })
    ];

    let connections = vec![
        graph.connect_nodes(nodes[0], nodes[1], Colour([0.0, 0.0, 1.0, 1.0])),
        graph.connect_nodes(nodes[2], nodes[0], Colour([1.0, 0.0, 1.0, 1.0]))
    ];

    let mut app = App {
        gl: GlGraphics::new(opengl),
        graph,
        nodes,
        connections
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(ref e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut glyphs);
        }
    }
}
