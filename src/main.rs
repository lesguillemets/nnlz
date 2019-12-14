use nannou::prelude::*;
use nnlz::*;

fn main() {
    nannou::app(|app: &App| {
        app.window(app.window_id())
            .unwrap()
            .set_inner_size_pixels(WIDTH_IN_PIX as u32, HEIGHT_IN_PIX as u32);
        Model::random(WIDTH_IN_CELLS, HEIGHT_IN_CELLS)
    })
    .update(update)
    .simple_window(view)
    .run();
}

const CELLSIZE: u32 = 2;
const WIDTH_IN_CELLS: u32 = 300;
const HEIGHT_IN_CELLS: u32 = 300;
const WIDTH_IN_PIX: u32 = WIDTH_IN_CELLS * CELLSIZE;
const HEIGHT_IN_PIX: u32 = HEIGHT_IN_CELLS * CELLSIZE;

const COLOUR_ALIVE: Rgb<u8> = LIMEGREEN;
const COLOUR_DEAD: Rgb<u8> = DARKOLIVEGREEN;

const RULE: Rule = Rule {
    birth_min: 3,
    birth_max: 3,
    alive_min: 2,
    alive_max: 3,
};

fn update(_app: &App, model: &mut Model, _update: Update) {
    // 普通のオートマトン
    let current = model.clone();
    for (i, &cell) in current.world.iter().enumerate() {
        let neighbours = current.neighbours_of(i as u32);
        if cell.is_alive() {
            if neighbours < RULE.alive_min || RULE.alive_max < neighbours {
                model.world[i] = Cell::Dead;
            }
        } else {
            // for dead cells
            if RULE.birth_min <= neighbours && neighbours <= RULE.birth_max {
                model.world[i] = Cell::Alive;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(COLOUR_DEAD);

    for (i, _cell) in model.world.iter().enumerate().filter(|(_, c)| c.is_alive()) {
        draw.rect()
            .x_y(
                // (i) 座標系として (0,0) が画面の中央にある
                // (ii) rect().x_y() も中心からの設定になる-のでこういう不明なのが必要
                // 多分設定できるんだけど見つからない
                ((CELLSIZE * (i as u32 % WIDTH_IN_CELLS) + CELLSIZE / 2) as i64
                    - WIDTH_IN_PIX as i64 / 2) as f32,
                ((CELLSIZE * (i as u32 / WIDTH_IN_CELLS) + CELLSIZE / 2) as i64
                    - HEIGHT_IN_PIX as i64 / 2) as f32,
            )
            .width(CELLSIZE as f32)
            .height(CELLSIZE as f32)
            .color(COLOUR_ALIVE);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
