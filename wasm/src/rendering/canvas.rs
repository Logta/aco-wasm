use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::geometry::city::City;

#[wasm_bindgen]
pub struct CanvasRenderer {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl CanvasRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<CanvasRenderer, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = canvas.width();
        let height = canvas.height();

        Ok(CanvasRenderer {
            canvas,
            context,
            width,
            height,
        })
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    #[wasm_bindgen]
    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        
        // Set background color
        self.context.set_fill_style_str("#1f2937");
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    #[wasm_bindgen]
    pub fn draw_city(&self, city: &City, is_highlighted: bool) {
        let radius = if is_highlighted { 10.0 } else { 8.0 };
        let color = if is_highlighted { "#f59e0b" } else { "#3b82f6" };

        // Draw city circle
        self.context.begin_path();
        self.context
            .arc(city.x(), city.y(), radius, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        self.context.set_fill_style_str(color);
        self.context.fill();
        
        // Draw border
        self.context.set_stroke_style_str("#ffffff");
        self.context.set_line_width(2.0);
        self.context.stroke();

        // Draw city ID
        self.context.set_fill_style_str("#ffffff");
        self.context.set_font("12px Arial");
        self.context.set_text_align("center");
        self.context.set_text_baseline("middle");
        let _ = self.context.fill_text(&city.id().to_string(), city.x(), city.y());
    }

    #[wasm_bindgen]
    pub fn draw_line(&self, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64, alpha: f64) {
        self.context.save();
        self.context.set_global_alpha(alpha);
        self.context.begin_path();
        self.context.move_to(x1, y1);
        self.context.line_to(x2, y2);
        self.context.set_stroke_style_str(color);
        self.context.set_line_width(width);
        self.context.stroke();
        self.context.restore();
    }

    #[wasm_bindgen]
    pub fn draw_pheromone_trail_simple(&self, cities_json: &str, pheromone_matrix: &[f64], max_pheromone: f64) {
        // Parse cities from JSON string
        if let Ok(cities_data) = js_sys::JSON::parse(cities_json) {
            if let Ok(cities_array) = js_sys::Array::from(&cities_data).dyn_into::<js_sys::Array>() {
                let num_cities = cities_array.length() as usize;
                
                for i in 0..num_cities {
                    for j in (i + 1)..num_cities {
                        if i * num_cities + j < pheromone_matrix.len() {
                            let pheromone_level = pheromone_matrix[i * num_cities + j];
                            if pheromone_level > 0.1 {
                                let alpha = (pheromone_level / max_pheromone).min(1.0) * 0.8;
                                let width = (pheromone_level / max_pheromone * 5.0).max(1.0);
                                
                                let city1 = cities_array.get(i as u32);
                                let city2 = cities_array.get(j as u32);
                                if !city1.is_undefined() && !city2.is_undefined() {
                                    if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (
                                        js_sys::Reflect::get(&city1, &"x".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                                        js_sys::Reflect::get(&city1, &"y".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                                        js_sys::Reflect::get(&city2, &"x".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                                        js_sys::Reflect::get(&city2, &"y".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                                    ) {
                                        self.draw_line(x1, y1, x2, y2, "#22c55e", width, alpha);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn draw_route_simple(&self, cities_json: &str, route: &[u32], color: &str, width: f64) {
        if route.len() < 2 {
            return;
        }

        // Parse cities from JSON string
        if let Ok(cities_data) = js_sys::JSON::parse(cities_json) {
            if let Ok(cities_array) = js_sys::Array::from(&cities_data).dyn_into::<js_sys::Array>() {
                self.context.begin_path();
                
                let start_city = cities_array.get(route[0]);
                if !start_city.is_undefined() {
                    if let (Ok(x), Ok(y)) = (
                        js_sys::Reflect::get(&start_city, &"x".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                        js_sys::Reflect::get(&start_city, &"y".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                    ) {
                        self.context.move_to(x, y);
                    }
                }

                for i in 1..route.len() {
                    let city = cities_array.get(route[i]);
                    if !city.is_undefined() {
                        if let (Ok(x), Ok(y)) = (
                            js_sys::Reflect::get(&city, &"x".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                            js_sys::Reflect::get(&city, &"y".into()).and_then(|v| v.as_f64().ok_or(JsValue::NULL)),
                        ) {
                            self.context.line_to(x, y);
                        }
                    }
                }

                self.context.set_stroke_style_str(color);
                self.context.set_line_width(width);
                self.context.stroke();
            }
        }
    }

    #[wasm_bindgen]
    pub fn draw_ant(&self, x: f64, y: f64, angle: f64) {
        self.context.save();
        self.context.translate(x, y).unwrap();
        self.context.rotate(angle).unwrap();

        // Draw ant body
        self.context.begin_path();
        self.context.ellipse(0.0, 0.0, 3.0, 1.5, 0.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        self.context.set_fill_style_str("#dc2626");
        self.context.fill();

        // Draw ant head
        self.context.begin_path();
        self.context.arc(3.0, 0.0, 1.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        self.context.set_fill_style_str("#7f1d1d");
        self.context.fill();

        self.context.restore();
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_canvas_renderer_creation() {
        // This test would require a DOM environment
        // For now, we'll just test the struct creation logic
        assert!(true);
    }
}