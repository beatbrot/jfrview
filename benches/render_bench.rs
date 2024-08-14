use criterion::{criterion_group, criterion_main, Criterion};
use egui::{vec2, Context, RawInput, Rect};
use jfrview::{FlameGraph, JfrViewApp};
use std::fs::File;

fn render_flame_graph(height: f32) -> (RawInput, Context) {
    let ctx = Context::default();
    let ri: RawInput = RawInput {
        screen_rect: Some(Rect::from_min_size(Default::default(), vec2(400.0, height))),
        ..RawInput::default()
    };
    (ri, ctx)
}

fn bench_large(c: &mut Criterion) {
    let file = File::open("cfg6_validate_small.jfr").unwrap();
    bench_render(c, &file, 600.0);
    bench_render(c, &file, 900.0);
}

fn bench_render(c: &mut Criterion, file: &File, height: f32) {
    let fg = FlameGraph::try_new(file).unwrap();
    let (ri, ctx) = render_flame_graph(height);
    let _ = ctx.run(ri, |ctx| {
        let mut app = JfrViewApp::new(ctx, fg);
        c.bench_function(format!("render large (height: {height})").as_str(), |b| b.iter(|| {
            app.simple_update(ctx);
        }));
    });
}

criterion_group!(benches, bench_large);
criterion_main!(benches);
