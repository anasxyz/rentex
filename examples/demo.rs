use rentex::{App, Ctx, Fonts, RentexApp};
use rentex::widgets::{ButtonWidget, WidgetHandle};

struct MyApp {
    btn: Option<WidgetHandle<ButtonWidget>>,
    counter: i32,
}

impl RentexApp for MyApp {
    fn setup(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("JetBrainsMono Nerd Font", 16.0);
        let btn = ctx.widgets.button("Click Me");
        ctx.widgets.get_mut(btn)
            .position(100.0, 100.0)
            .font(font)
            .auto_size()
            .color([0.0, 0.0, 1.0, 1.0]);
        self.btn = Some(btn);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        let btn = self.btn.unwrap();
        if ctx.widgets.get(btn).just_clicked {
            self.counter += 1;
            ctx.widgets.get_mut(btn).text(format!("Clicked: {}", self.counter));
        }
        if ctx.widgets.get(btn).just_hovered {
            ctx.widgets.get_mut(btn).color([0.0, 0.0, 0.6, 1.0]);
        }
        if ctx.widgets.get(btn).just_unhovered {
            ctx.widgets.get_mut(btn).color([0.0, 0.0, 1.0, 1.0]);
        }
    }
}

fn main() {
    App::new("RNTX demo", 800, 600).run(Fonts::new(), MyApp {
        btn: None,
        counter: 0,
    });
}
