extern crate wasm_bindgen;

use std::f64;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().request_animation_frame(f.as_ref().unchecked_ref()).expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window().document().expect("should have a document on window")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {



        context.set_fill_style(&"#CCCCCC".into());
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        let center_x = canvas.width() as f64/2.0;
        let center_y = canvas.height() as f64/2.0;
        let radius = 4.0;
        context.set_fill_style(&"#333333".into());
        context.begin_path();
        context.ellipse(center_x, center_y, radius, radius, 0.0, 0.0, 180.0).unwrap();
        context.fill();

        let stem_len = 6;
        let mut cnt = 0;
        while cnt < stem_len {
            let dist = std::cmp::min(i*3, 100) as f64;
            let degree = 360.0 * (cnt as f64 / 6.0);
            let theta = degree * std::f64::consts::PI / 180.0;
            let child_x = dist*theta.cos() + center_x;
            let child_y = dist*theta.sin() + center_y;
            context.begin_path();
            context.move_to(center_x, center_y);
            context.line_to(child_x, child_y);
            context.stroke();

            if i >= 33 {
                context.begin_path();
                context.ellipse(child_x, child_y, radius, radius, 0.0, 0.0, 180.0).unwrap();
                context.fill();

                let parent_x = child_x;
                let parent_y = child_y;
                let mut cnt_inner = 0;
                while cnt_inner < stem_len {
                    let dist = std::cmp::min(i*3, (dist*2.0) as i32) as f64;
                    let degree = degree + (60.0*(cnt_inner as f64/6.0));
                    let theta = degree * std::f64::consts::PI / 180.0;
                    let child_x = dist*theta.cos() + center_x;
                    let child_y = dist*theta.sin() + center_y;
                    context.begin_path();
                    context.move_to(parent_x, parent_y);
                    context.line_to(child_x, child_y);
                    context.stroke();

                    if i >= 66 {
                        context.begin_path();
                        context.ellipse(child_x, child_y, radius, radius, 0.0, 0.0, 180.0).unwrap();
                        context.fill();

                        let parent_x = child_x;
                        let parent_y = child_y;
                        let mut cnt_inner2 = 0;
                        while cnt_inner2 < stem_len {
                            let dist = std::cmp::min(i*3, (dist*2.0) as i32) as f64;
                            let degree = degree + (10.0*(cnt_inner2 as f64/6.0));
                            let theta = degree * std::f64::consts::PI / 180.0;
                            let child_x = dist*theta.cos() + center_x;
                            let child_y = dist*theta.sin() + center_y;
                            context.begin_path();
                            context.move_to(parent_x, parent_y);
                            context.line_to(child_x, child_y);
                            context.stroke();

                            if i >= 132 {
                                context.begin_path();
                                context.ellipse(child_x, child_y, radius, radius, 0.0, 0.0, 180.0).unwrap();
                                context.fill();
                            }
                            cnt_inner2 += 1;
                        }
                    }
                    cnt_inner += 1;
                }
            }
            cnt += 1;
        }

        i += 1;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
