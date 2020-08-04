extern crate wasm_bindgen;

use std::f64;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

enum Step {
    C, D, E, F, G, A, B
}
use Step::*;
impl Step {
    fn scale_index(&self) -> u32 {
        match self {
            C => 0, D => 1, E => 2, F => 3, G => 4, A => 5, B => 6,
        }
    }
    fn chromatic_index(&self) -> u32 {
        match self {
            C => 0, D => 2, E => 4, F => 5, G => 7, A => 9, B => 11,
        }
    }
    fn next_step(&self) -> Step {
        match self {
            C => D, D => E, E => F, F => G, G => A, A => B, B => C,
        }
    }
    fn to_str(&self) -> &str {
        match self {
            C => "C", D => "D", E => "E", F => "F", G => "G", A => "A", B => "B",
        }
    }
}

enum Alter {
    Flat,
    Natural,
    Sharp,
}
use Alter::*;
impl Alter {
    fn value(&self) -> i32 {
        match self {
            Flat => -1,
            Natural => 0,
            Sharp => 1,
        }
    }
    fn clone(&self) -> Alter {
        self.clone()
    }
    fn get_minos(&self) -> Alter {
        match self {
            Natural => Flat,
            Sharp => Natural,
            _ => panic!()
        }
    }
    fn get_plus(&self) -> Alter {
        match self {
            Flat => Natural,
            Natural => Sharp,
            _ => panic!()
        }
    }
    fn to_str(&self) -> &str {
        match self {
            Flat => " Flat ",
            Natural => " ",
            Sharp => " Sharp ",
        }
    }
}

struct Octave(u32);
impl Octave {
    fn value(&self) -> u32 {
        self.0
    }
    /*fn clone(&self) -> u32 {
        self.0.clone()
    }*/
}

struct Note {
    step: Step,
    alter: Alter,
    octave: Octave,
}
impl Note {
    fn clone(&self) -> Note {
        self.clone()
    }
}

enum KeyType {
    Major,
    Minor
}
impl KeyType {
    fn clone(&self) -> KeyType {
        self.clone()
    }
    fn to_str(&self) -> &str {
        match self {
            Major => "Major",
            Minor => "Minor",
        }
    }
    fn from_str(type_str: &str) -> KeyType {
        match type_str {
            "Major" => KeyType::Major,
            "Minor" => KeyType::Minor,
        }
    }
}

struct Key {
    root_note: Note,
    key_type: KeyType,
}

fn root_key() -> Key {
    let root_step = Step::C;
    let root_alter = Alter::Natural;
    let root_key_type = KeyType::Major;
    init_root_key(root_step, root_alter, root_key_type)
}

fn key_to_str(key: &Key) -> String {
    let step = key.root_note.step.to_str();
    let alter = key.root_note.alter.to_str();
    let key_type = key.key_type.to_str();
    format!("{}{}{}", step, alter, key_type)
}

fn init_root_key(step: Step, alter: Alter, key_type: KeyType) -> Key {
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
    /*match root_key.key_type {
        KeyType::Major => related_keys_of_major(&root_key),
        KeyType::Minor => related_keys_of_minor(&root_key),
    }*/
    related_keys_of_major(&root_key)
}

fn related_keys_of_major(root_key: &Key) -> Vec<Key> {
    let mut result: Vec<Key> = Vec::new();
    let chromatic_dist_arr = [0, 2, 4, 5, 7, 9];
    let key_type_arr = ["Minor", "Minor", "Minor", "Major", "Major", "Minor"];
    let mut current_step = &root_key.root_note.step;
    for i in 0..chromatic_dist_arr.len() {

        let mut next_octave = 0;
        if root_key.root_note.step.scale_index() == 6 {
            next_octave += 1;
        }

        let next_note = Note {
            step: root_key.root_note.step.next_step(),
            alter: root_key.root_note.alter.clone(),
            octave: Octave(next_octave),
        };

        let next_key = Key {
            root_note: next_note,
            key_type: KeyType::from_str(key_type_arr[i]),
        };
        result.push(next_key);
        current_step = &root_key.root_note.step.next_step();
    }
    result
}

/*fn related_keys_of_minor(root_key: &Key) -> Vec<Key> {
    let mut result: Vec<Key> = Vec::new();
    let chromatic_dist_arr = [0, 2, 1, 2, 2, 1, 2];
    let key_type_arr = [KeyType::Major, KeyType::Major, KeyType::Minor, KeyType::Minor, KeyType::Major, KeyType::Major];
    let rkrn = &root_key.root_note;
    for i in 0..chromatic_dist_arr.len() {

        if i == 1 { continue };

        let dist = chromatic_dist_arr[i];
        let dist_minus = dist-1;
        let dist_plus = dist+1;

        let next_step = rkrn.step.next_step();
        let mut next_octave = rkrn.octave.clone();
        if rkrn.step.scale_index() == 6 {
            next_octave += 1;
        }
        let mut next_note = Note {
            step: next_step,
            alter: rkrn.alter.clone(),
            octave: Octave(next_octave),
        };

        next_note.alter = match chromatic_interval(&rkrn, &next_note) as i32 {
            dist_minus => next_note.alter.get_plus(),
            dist => next_note.alter,
            dist_plus => next_note.alter.get_minos(),
        };

        let next_key = Key {
            root_note: next_note.clone(),
            key_type: key_type_arr[i].clone(),
        };
        result.push(next_key);
    }
    result
}*/

const OCTAVE_CHROMATIC_STEPS: u32 = 12;
fn chromatic_interval(n1: &Note, n2: &Note) -> u32 {
    let n1_oct = n1.octave.value() * OCTAVE_CHROMATIC_STEPS;
    let n2_oct = n2.octave.value() * OCTAVE_CHROMATIC_STEPS;
    let n1_steps = (n1_oct + n1.step.chromatic_index()) as i32 + n1.alter.value();
    let n2_steps = (n2_oct + n2.step.chromatic_index()) as i32 + n2.alter.value();
    (n1_steps as i32 - n2_steps as i32).abs() as u32 % OCTAVE_CHROMATIC_STEPS
}








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

    //keys();

    let root_key = root_key();
    let rk_str = key_to_str(&root_key);

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
        //context.begin_path();
        //context.ellipse(center_x, center_y, radius, radius, 0.0, 0.0, 180.0).unwrap();
        //context.fill();
        context.set_font("bold 28px sans-serif");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.fill_text(&rk_str, center_x, center_y).unwrap();

        let related_keys = related_keys(&root_key);
        for i in 0..6 {
            let key = &related_keys[i];
            context.fill_text(&key_to_str(key), 100.0, 100.0 + (i*100)as f64).unwrap();
        }


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
