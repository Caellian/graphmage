use glam::Vec2;
use graphmage_graph::{
    edge::{Attachment, EdgeData},
    node::Node,
    Graph,
};
use graphmage_render::draw;
use window::Window;

mod window;

fn main() {
    /*let source = match std::env::args().nth(1) {
        Some(it) => it,
        None => {
            // TODO: Show an open dialog instead.
            eprintln!("Please provide a path to a graphmage file as an argument.");
            exit(1)
        }
    };*/

    let window = Window::new(800, 600);

    let mut g = Graph::new();

    let mut a = Node::new(Vec2::new(50.0, 50.0));
    a.properties.set_label("Hello".to_string());
    a.properties.set_font_size(20.0);
    let a = g.push_node(a);

    let mut b = Node::new(Vec2::new(120.0, 100.0));
    b.properties.set_label("world".to_string());
    let b = g.push_node(b);

    let mut c = Node::new(Vec2::new(300.0, 50.0));
    c.properties
        .set_label("And a really long label just to see what happens :)".to_string());
    let c = g.push_node(c);

    g.push_edge(
        Attachment {
            node: a,
            port: None,
        },
        Attachment {
            node: b,
            port: None,
        },
        EdgeData::default(),
    );

    window.run(|ctx| draw(ctx, &g));
}
