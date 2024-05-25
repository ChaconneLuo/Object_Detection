use image::{imageops::FilterType, GenericImageView};
use ndarray::{s, Array, Axis, IxDyn};
use ort::{Environment, SessionBuilder, Value};
use std::{sync::Arc, vec};

#[tauri::command]
pub fn detect(algorithm: &str, model: &str, img: &str, iou_value: f32, pred_value: f32) -> String {
    let buf = std::fs::read(img).unwrap_or(vec![]);
    let boxes = detect_objects_on_image(buf, algorithm, model, iou_value, pred_value);
    return serde_json::to_string(&boxes).unwrap_or_default();
}

pub fn detect_objects_on_image(
    buf: Vec<u8>,
    algorithm: &str,
    model: &str,
    iou_value: f32,
    pred_value: f32,
) -> Vec<(f32, f32, f32, f32, &'static str, f32)> {
    let (input, img_width, img_height) = prepare_input(buf);
    let output = run_model(input, algorithm, model);
    return process_output(output, img_width, img_height, iou_value, pred_value);
}

// Returns the input tensor, original image width and height
fn prepare_input(buf: Vec<u8>) -> (Array<f32, IxDyn>, u32, u32) {
    let img = image::load_from_memory(&buf).unwrap();
    let (img_width, img_height) = (img.width(), img.height());
    let img = img.resize_exact(640, 640, FilterType::CatmullRom);
    let mut input = Array::zeros((1, 3, 640, 640)).into_dyn();
    for pixel in img.pixels() {
        let x = pixel.0 as usize;
        let y = pixel.1 as usize;
        let [r, g, b, _] = pixel.2 .0;
        input[[0, 0, y, x]] = (r as f32) / 255.0;
        input[[0, 1, y, x]] = (g as f32) / 255.0;
        input[[0, 2, y, x]] = (b as f32) / 255.0;
    }
    return (input, img_width, img_height);
}

fn run_model(input: Array<f32, IxDyn>, algorithm: &str, model: &str) -> Array<f32, IxDyn> {
    let env = Arc::new(Environment::builder().with_name("YOLOv8").build().unwrap());
    let model = SessionBuilder::new(&env)
        .unwrap()
        .with_model_from_file(format!("../data/{}/weights/{}", algorithm, model))
        .unwrap();
    let input_as_values = &input.as_standard_layout();
    let model_inputs: Vec<Value> =
        vec![Value::from_array(model.allocator(), input_as_values).unwrap()];
    let outputs = model.run(model_inputs).unwrap();
    let output = outputs
        .get(0)
        .unwrap()
        .try_extract::<f32>()
        .unwrap()
        .view()
        .t()
        .into_owned();
    return output;
}

fn process_output(
    output: Array<f32, IxDyn>,
    img_width: u32,
    img_height: u32,
    iou_value: f32,
    pred_value: f32,
) -> Vec<(f32, f32, f32, f32, &'static str, f32)> {
    let mut boxes = Vec::new();
    let output = output.slice(s![.., .., 0]);
    for row in output.axis_iter(Axis(0)) {
        let row: Vec<_> = row.iter().map(|x| *x).collect();
        let (class_id, prob) = row
            .iter()
            .skip(4)
            .enumerate()
            .map(|(index, value)| (index, *value))
            .reduce(|accum, row| if row.1 > accum.1 { row } else { accum })
            .unwrap();
        if prob < pred_value {
            continue;
        }
        println!("{} {}", class_id % 2, prob);
        let label = YOLO_FIRE_CLASSES[class_id%2];
        let xc = row[0] / 640.0 * (img_width as f32);
        let yc = row[1] / 640.0 * (img_height as f32);
        let w = row[2] / 640.0 * (img_width as f32);
        let h = row[3] / 640.0 * (img_height as f32);
        let x1 = xc - w / 2.0;
        let x2 = xc + w / 2.0;
        let y1 = yc - h / 2.0;
        let y2 = yc + h / 2.0;
        println!("{} {} {} {} ", x1, y1, x2, y2);
        boxes.push((x1, y1, x2, y2, label, prob));
    }

    boxes.sort_by(|box1, box2| box2.5.total_cmp(&box1.5));
    let mut result = Vec::new();
    while boxes.len() > 0 {
        result.push(boxes[0]);
        boxes = boxes
            .iter()
            .filter(|box1| iou(&boxes[0], box1) < iou_value)
            .map(|x| *x)
            .collect()
    }
    return result;
}

fn iou(
    box1: &(f32, f32, f32, f32, &'static str, f32),
    box2: &(f32, f32, f32, f32, &'static str, f32),
) -> f32 {
    return intersection(box1, box2) / union(box1, box2);
}

fn union(
    box1: &(f32, f32, f32, f32, &'static str, f32),
    box2: &(f32, f32, f32, f32, &'static str, f32),
) -> f32 {
    let (box1_x1, box1_y1, box1_x2, box1_y2, _, _) = *box1;
    let (box2_x1, box2_y1, box2_x2, box2_y2, _, _) = *box2;
    let box1_area = (box1_x2 - box1_x1) * (box1_y2 - box1_y1);
    let box2_area = (box2_x2 - box2_x1) * (box2_y2 - box2_y1);
    return box1_area + box2_area - intersection(box1, box2);
}


fn intersection(
    box1: &(f32, f32, f32, f32, &'static str, f32),
    box2: &(f32, f32, f32, f32, &'static str, f32),
) -> f32 {
    let (box1_x1, box1_y1, box1_x2, box1_y2, _, _) = *box1;
    let (box2_x1, box2_y1, box2_x2, box2_y2, _, _) = *box2;
    let x1 = box1_x1.max(box2_x1);
    let y1 = box1_y1.max(box2_y1);
    let x2 = box1_x2.min(box2_x2);
    let y2 = box1_y2.min(box2_y2);
    return (x2 - x1) * (y2 - y1);
}

const YOLO_FIRE_CLASSES: [&str; 2] = ["fire", "fire_small"];
