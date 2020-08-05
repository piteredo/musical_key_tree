extern crate wasm_bindgen;

use std::f64;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const OCTAVE_CHROMATIC_STEPS: u32 = 12;


//------------------------------------------------------------


enum Step { C, D, E, F, G, A, B }
use Step::*;
impl Step {
    fn scale_index(&self) -> u32 {
        match self { C => 0, D => 1, E => 2, F => 3, G => 4, A => 5, B => 6,}
    }
    fn chromatic_index(&self) -> u32 {
        match self { C => 0, D => 2, E => 4, F => 5, G => 7, A => 9, B => 11,}
    }
    fn to_str(&self) -> &str {
        match self { C => "C", D => "D", E => "E", F => "F", G => "G", A => "A", B => "B",}
    }
    fn from_num(n: i32) -> Step {
        match n%7 { 0 => C, 1 => D, 2 => E, 3 => F, 4 => G, 5 => A, 6 => B, _=> panic!(),}
    }
}

enum Alter { Flat, Natural, Sharp, }
use Alter::*;
impl Alter {
    fn value(&self) -> i32 {
        match self { Flat => -1, Natural => 0, Sharp => 1,}
    }
    fn clone(&self) -> Alter {
        match self { Flat => Flat, Natural => Natural, Sharp => Sharp,}
    }
    fn get_minos(&self) -> Alter {
        match self { Natural => Flat, Sharp => Natural, _ => panic!()}
    }
    fn get_plus(&self) -> Alter {
        match self { Flat => Natural, Natural => Sharp, _ => panic!()}
    }
    fn to_str(&self) -> &str {
        match self { Flat => "b", Natural => "", Sharp => "#",}
    }
}

struct Octave(u32);
impl Octave {
    fn value(&self) -> u32 { self.0 }
    fn clone(&self) -> u32 { self.0.clone() }
}

enum KeyType { Major, Minor }
impl KeyType {
    fn to_str(&self) -> &str {
        match self { KeyType::Major => "", KeyType::Minor => "m",}
    }
    fn from_str(ty: i32) -> KeyType {
        match ty { 0 => KeyType::Major, 1 => KeyType::Minor, _ => panic!() }
    }
}

struct Note { step: Step, alter: Alter, octave: Octave, }
struct Key { root_note: Note, key_type: KeyType, }


//------------------------------------------------------------


fn window() -> web_sys::Window {
    web_sys::window()
        .expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}


//------------------------------------------------------------


fn key_to_str(key: &Key) -> String {
    let step = key.root_note.step.to_str();
    let alter = key.root_note.alter.to_str();
    let key_type = key.key_type.to_str();
    format!("{}{}{}", step, alter, key_type)
}

fn init_key(step: Step, alter: Alter, key_type: KeyType) -> Key {
    let root_note = Note {
        step: step,
        alter: alter,
        octave: Octave(0),
    };
    Key {
        root_note: root_note,
        key_type: key_type,
    }
}

fn related_keys(root_key: &Key) -> Vec<Key> {
    let mut result: Vec<Key> = Vec::new();
    let chromatic_dist_arr = match root_key.key_type {
        KeyType::Major => [0, 2, 4, 5, 7, 9, 11],
        KeyType::Minor => [0, 2, 3, 5, 7, 8, 10],
    };
    let key_type_arr = match root_key.key_type {
        KeyType::Major => [1, 1, 1, 0, 0, 1, 0],
        KeyType::Minor => [0, 0, 0, 1, 1, 0, 0],
    };
    let pass_step = match root_key.key_type {
        KeyType::Major => 6,
        KeyType::Minor => 1,
    };
    for i in 0..chromatic_dist_arr.len() {
        if i == pass_step { continue; };

        let chrom_dist = chromatic_dist_arr[i];

        let mut next_octave = root_key.root_note.octave.clone();
        if (root_key.root_note.step.scale_index() + i as u32) >= 7 {
            next_octave += 1;
        }

        let mut next_note = Note {
            step: Step::from_num((root_key.root_note.step.scale_index() + i as u32) as i32),
            alter: root_key.root_note.alter.clone(),
            octave: Octave(next_octave),
        };

        let dist = chromatic_interval(&root_key.root_note, &next_note);
        if dist > chrom_dist {
            next_note.alter = next_note.alter.get_minos();
        } else if dist < chrom_dist {
            next_note.alter = next_note.alter.get_plus();
        }

        let next_key = Key {
            root_note: next_note,
            key_type: KeyType::from_str(key_type_arr[i]),
        };

        result.push(next_key);
    }
    result
}

fn chromatic_interval(n1: &Note, n2: &Note) -> u32 {
    let n1_oct = n1.octave.value() * OCTAVE_CHROMATIC_STEPS;
    let n2_oct = n2.octave.value() * OCTAVE_CHROMATIC_STEPS;
    let n1_steps = (n1_oct + n1.step.chromatic_index()) as i32 + n1.alter.value();
    let n2_steps = (n2_oct + n2.step.chromatic_index()) as i32 + n2.alter.value();
    (n1_steps as i32 - n2_steps as i32).abs() as u32 % OCTAVE_CHROMATIC_STEPS
}

fn degree(parent_degree: f64, circle_cnt: usize, circle_num: i32, stem_len: usize) -> f64{
    parent_degree + ((360.0 / circle_num as f64) * (circle_cnt as f64 / stem_len as f64))
}

fn goal_pos(frame_count: i32, speed: i32, circle_1_radius: i32, degree: f64, parent_x: f64, parent_y: f64) -> Vec<f64> {
    let dist = std::cmp::min(frame_count*speed, circle_1_radius) as f64;
    let theta = degree * std::f64::consts::PI / 180.0;
    let goal_x = dist*theta.cos() + parent_x;
    let goal_y = dist*theta.sin() + parent_y;
    vec![goal_x, goal_y]
}


//------------------------------------------------------------


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.set_stroke_style(&"rgb(100,100,100,0.3)".into());
    ctx.set_font("bold 18px sans-serif");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    let mut key_cheker1 = std::collections::HashMap::new();
    let mut key_cheker2 = std::collections::HashMap::new();

    let root_step = Step::C;
    let root_alter = Alter::Natural;
    let root_key_type = KeyType::Minor;
    let root_key = init_key(root_step, root_alter, root_key_type);
    let root_key_str = key_to_str(&root_key);
    key_cheker1.insert(root_key_str, 0);

    let center_x = canvas.width() as f64/2.0;
    let center_y = canvas.height() as f64/2.0;
    let mut frame_count = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        // bg
        ctx.set_fill_style(&"#DDDDBB".into());
        ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        ctx.set_fill_style(&"#333333".into()); // for text

        //--------------------------

        let circle_1_radius = 100;
        let circle_2_radius = 200;
        let circle_3_radius = 400;

        //TODO
        ctx.fill_text(&key_to_str(&root_key), center_x, center_y).unwrap();
        let parent_x = center_x;
        let parent_y = center_y;

        let keys1 = related_keys(&root_key);
        for i in 0..keys1.len() {
            let key1 = keys1.get(i).unwrap();
            let key1_str = key_to_str(key1);
            key_cheker1.insert(key1_str, 0);

            //TODO
            let parent_degree = 0.0;
            let degree = degree(parent_degree, i, 1, keys1.len()); // 1 => circle_level
            let goal_pos = goal_pos(frame_count, speed, circle_1_radius, degree, parent_x, parent_y);
            ctx.begin_path();
            ctx.move_to(parent_x, parent_y);
            ctx.line_to(goal_pos[0], goal_pos[1]);
            ctx.stroke();
            ctx.fill_text(&key_to_str(key1), 150.0, 100.0+(i*50)as f64).unwrap();

            let keys2 = related_keys(key1);
            for j in 0..keys2.len() {
                let key2 = keys2.get(j).unwrap();
                let key2_str = key_to_str(key2);
                if !key_cheker1.contains_key(&key2_str) {
                    key_cheker2.insert(key2_str, 0);

                    //TODO
                    ctx.fill_text(&key_to_str(key2), 200.0+(i*50)as f64, 100.0+(j*50)as f64).unwrap();

                    let keys3 = related_keys(key2);
                    for k in 0..keys3.len() {
                        let key3 = keys3.get(k).unwrap();
                        let key3_str = key_to_str(key3);
                        if !key_cheker1.contains_key(&key3_str) && !key_cheker2.contains_key(&key3_str) {

                            //TODO
                            ctx.fill_text(&key_to_str(key3), 100.0+(i*550)as f64+(j*50)as f64, 500.0+(k*50)as f64).unwrap();
                        }
                    }
                }
            }
        };









        /*let speed = 7;
        let circle_1_radius = 100;
        let circle_2_radius = 200;
        let circle_3_radius = 400;

        let related_keys = related_keys(&root_key);
        let stem_len = related_keys.len() as i32;

        let parent_x = center_x;
        let parent_y = center_y;
        let mut circle_1_list: Vec<[f64; 3]> = Vec::new();
        let mut circle_2_list: Vec<[f64; 3]> = Vec::new();
        let mut circle_1_cnt = 0;
        while circle_1_cnt < stem_len {
            let parent_degree = 0.0;
            let degree = degree(parent_degree, circle_1_cnt, 1, stem_len); // 1 => circle_level
            let goal_pos = goal_pos(frame_count, speed, circle_1_radius, degree, parent_x, parent_y);
            ctx.begin_path();
            ctx.move_to(parent_x, parent_y);
            ctx.line_to(goal_pos[0], goal_pos[1]);
            ctx.stroke();
            circle_1_list.push([degree, goal_pos[0], goal_pos[1]]);

            let key = related_keys.get(circle_1_cnt as usize).unwrap();
            ctx.fill_text(&key_to_str(key), goal_pos[0], goal_pos[1]).unwrap();

            circle_1_cnt += 1;
        }
        ctx.fill_text(&rk_str, center_x, center_y).unwrap();

        if frame_count >= circle_1_radius / speed {
            for ii in 0..circle_1_list.len() {
                let parent_x = circle_1_list[ii][1];
                let parent_y = circle_1_list[ii][2];
                let mut circle_2_cnt = 0;
                while circle_2_cnt < stem_len {
                    let parent_degree = circle_1_list[ii][0];
                    let degree = degree(parent_degree, circle_2_cnt, 6, stem_len); // 6 => circle_level
                    let goal_pos = goal_pos(frame_count, speed, circle_2_radius, degree, center_x, center_y);
                    ctx.begin_path();
                    ctx.move_to(parent_x, parent_y);
                    ctx.line_to(goal_pos[0], goal_pos[1]);
                    ctx.stroke();
                    circle_2_list.push([degree, goal_pos[0], goal_pos[1]]);

                    if frame_count >= circle_1_radius/speed + circle_2_radius/speed {

                    }
                    circle_2_cnt += 1;
                }
            }
        }

        if frame_count >= circle_1_radius/speed + circle_2_radius/speed {
            for ii in 0..circle_2_list.len() {
                let parent_x = circle_2_list[ii][1];
                let parent_y = circle_2_list[ii][2];
                let mut circle_3_cnt = 0;
                while circle_3_cnt < stem_len {
                    let parent_degree = circle_2_list[ii][0];
                    let degree = degree(parent_degree, circle_3_cnt, 36, stem_len); // 36 => circle_level
                    let goal_pos = goal_pos(frame_count, speed, circle_3_radius, degree, center_x, center_y);
                    ctx.begin_path();
                    ctx.move_to(parent_x, parent_y);
                    ctx.line_to(goal_pos[0], goal_pos[1]);
                    ctx.stroke();

                    if frame_count >= circle_1_radius/speed + circle_2_radius/speed {

                    }
                    circle_3_cnt += 1;
                }
            }
        }*/

        frame_count += 1;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
