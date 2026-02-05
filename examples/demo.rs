// examples/complete_demo.rs
use rentex::App;

fn main() {
    let app = App::new("Complete Feature Demo", 1000, 700);

    app.run(|rntx| {
        // Title
        rntx.scene
            .text("Rentex Feature Showcase", 20.0, 20.0)
            .font_size(36.0)
            .color([1.0, 1.0, 0.0, 1.0]);

        // Section 1: Basic Shapes with Outlines
        rntx.scene
            .text("1. Shapes with Outlines", 20.0, 80.0)
            .font_size(20.0)
            .color([0.8, 0.8, 0.8, 1.0]);

        // Rectangle with outline
        rntx.scene
            .rect(20.0, 110.0, 120.0, 80.0)
            .fill_color([0.8, 0.2, 0.2, 1.0])
            .outline_color([1.0, 1.0, 1.0, 1.0])
            .outline_width(3.0);

        // Circle with outline
        rntx.scene
            .circle(220.0, 150.0, 40.0)
            .fill_color([0.2, 0.8, 0.2, 1.0])
            .outline_color([0.0, 0.0, 0.0, 1.0])
            .outline_width(4.0);

        // Rounded rect with outline
        rntx.scene
            .rounded_rect(280.0, 110.0, 120.0, 80.0, 15.0)
            .fill_color([0.2, 0.2, 0.8, 1.0])
            .outline_color([1.0, 0.5, 0.0, 1.0])
            .outline_width(2.0);

        // Section 2: Text with Custom Colors
        rntx.scene
            .text("2. Text Colors", 20.0, 220.0)
            .font_size(20.0)
            .color([0.8, 0.8, 0.8, 1.0]);

        rntx.scene
            .text("Red Text", 20.0, 250.0)
            .font_size(18.0)
            .color([1.0, 0.0, 0.0, 1.0]);

        rntx.scene
            .text("Green Text", 120.0, 250.0)
            .font_size(18.0)
            .color([0.0, 1.0, 0.0, 1.0]);

        rntx.scene
            .text("Blue Text", 240.0, 250.0)
            .font_size(18.0)
            .color([0.0, 0.5, 1.0, 1.0]);

        rntx.scene
            .text("Yellow Text", 350.0, 250.0)
            .font_size(18.0)
            .color([1.0, 1.0, 0.0, 1.0]);

        // Section 3: Interactive Buttons
        rntx.scene
            .text("3. Interactive Buttons (click & hover)", 20.0, 300.0)
            .font_size(20.0)
            .color([0.8, 0.8, 0.8, 1.0]);

        // Button with click handler
        rntx.scene
            .button(20.0, 330.0, 180.0, 50.0, "Click Me!")
            .fill_color([0.2, 0.6, 1.0, 1.0])
            .text_color([1.0, 1.0, 1.0, 1.0])
            .hover_color([0.3, 0.7, 1.0, 1.0])
            .outline_color([0.0, 0.0, 0.0, 1.0])
            .outline_width(2.0)
            .on_click(|| {
                println!("Button 1 clicked!");
            })
            .on_hover(|entered| {
                if entered {
                    println!("Hovering button 1");
                }
            });

        // Button with different style
        rntx.scene
            .button(220.0, 330.0, 180.0, 50.0, "Press Me!")
            .fill_color([0.8, 0.2, 0.2, 1.0])
            .text_color([1.0, 1.0, 1.0, 1.0])
            .hover_color([1.0, 0.3, 0.3, 1.0])
            .on_click(|| {
                println!("Button 2 clicked!");
            });

        // Outlined button
        rntx.scene
            .button(420.0, 330.0, 180.0, 50.0, "Outlined")
            .fill_color([0.1, 0.1, 0.1, 1.0])
            .text_color([1.0, 1.0, 1.0, 1.0])
            .outline_color([0.0, 1.0, 0.5, 1.0])
            .outline_width(3.0)
            .hover_color([0.2, 0.2, 0.2, 1.0])
            .on_click(|| {
                println!("Button 3 clicked!");
            });

        // Section 4: UI Card Example
        rntx.scene
            .text("4. UI Card Example", 20.0, 410.0)
            .font_size(20.0)
            .color([0.8, 0.8, 0.8, 1.0]);

        // Card background
        rntx.scene
            .rounded_rect(20.0, 440.0, 350.0, 220.0, 15.0)
            .fill_color([0.15, 0.15, 0.15, 1.0])
            .outline_color([0.3, 0.3, 0.3, 1.0])
            .outline_width(2.0);

        // Card title
        rntx.scene
            .text("Dashboard Card", 40.0, 460.0)
            .font_size(24.0)
            .color([1.0, 1.0, 1.0, 1.0]);

        // Card description
        rntx.scene
            .text("This demonstrates a more complex", 40.0, 495.0)
            .font_size(16.0)
            .color([0.7, 0.7, 0.7, 1.0]);

        rntx.scene
            .text("UI component with multiple elements.", 40.0, 520.0)
            .font_size(16.0)
            .color([0.7, 0.7, 0.7, 1.0]);

        // Status indicator
        rntx.scene
            .circle(340.0, 470.0, 8.0)
            .fill_color([0.0, 1.0, 0.0, 1.0])
            .outline_color([0.0, 0.5, 0.0, 1.0])
            .outline_width(1.5);

        rntx.scene
            .text("Active", 300.0, 463.0)
            .font_size(14.0)
            .color([0.0, 1.0, 0.0, 1.0]);

        // Card actions
        rntx.scene
            .button(40.0, 590.0, 140.0, 45.0, "View Details")
            .fill_color([0.2, 0.6, 1.0, 1.0])
            .hover_color([0.3, 0.7, 1.0, 1.0])
            .on_click(|| {
                println!("View Details clicked!");
            });

        rntx.scene
            .button(200.0, 590.0, 140.0, 45.0, "Settings")
            .fill_color([0.4, 0.4, 0.4, 1.0])
            .hover_color([0.5, 0.5, 0.5, 1.0])
            .outline_color([0.6, 0.6, 0.6, 1.0])
            .outline_width(1.5)
            .on_click(|| {
                println!("Settings clicked!");
            });

        // Section 5: Progress Indicator
        rntx.scene
            .text("5. Progress Example", 400.0, 410.0)
            .font_size(20.0)
            .color([0.8, 0.8, 0.8, 1.0]);

        // Progress bar background
        rntx.scene
            .rounded_rect(400.0, 450.0, 300.0, 30.0, 15.0)
            .fill_color([0.2, 0.2, 0.2, 1.0])
            .outline_color([0.4, 0.4, 0.4, 1.0])
            .outline_width(1.0);

        // Progress bar fill (75%)
        rntx.scene
            .rounded_rect(400.0, 450.0, 225.0, 30.0, 15.0)
            .fill_color([0.2, 0.8, 0.4, 1.0]);

        rntx.scene
            .text("75%", 650.0, 456.0)
            .font_size(16.0)
            .color([0.7, 0.7, 0.7, 1.0]);

        // Section 6: Icon Buttons
        rntx.scene
            .text("6. Icon-Style Buttons", 400.0, 510.0)
            .font_size(20.0)
            .color([0.8, 0.8, 0.8, 1.0]);

        // Small icon buttons
        rntx.scene
            .button(400.0, 545.0, 50.0, 50.0, "+")
            .fill_color([0.2, 0.8, 0.2, 1.0])
            .hover_color([0.3, 0.9, 0.3, 1.0])
            .on_click(|| println!("Plus clicked!"));

        rntx.scene
            .button(465.0, 545.0, 50.0, 50.0, "-")
            .fill_color([0.8, 0.2, 0.2, 1.0])
            .hover_color([0.9, 0.3, 0.3, 1.0])
            .on_click(|| println!("Minus clicked!"));

        rntx.scene
            .button(530.0, 545.0, 50.0, 50.0, "×")
            .fill_color([0.6, 0.6, 0.6, 1.0])
            .hover_color([0.7, 0.7, 0.7, 1.0])
            .on_click(|| println!("Close clicked!"));

        rntx.scene
            .button(595.0, 545.0, 50.0, 50.0, "↻")
            .fill_color([0.2, 0.4, 0.8, 1.0])
            .hover_color([0.3, 0.5, 0.9, 1.0])
            .on_click(|| println!("Refresh clicked!"));

        // Input info
        let mouse_pos = rntx.input.mouse_position;
        rntx.scene
            .text(
                &format!("Mouse: ({:.0}, {:.0})", mouse_pos.0, mouse_pos.1),
                750.0,
                20.0,
            )
            .font_size(14.0)
            .color([0.5, 0.5, 0.5, 1.0]);
    });
}
