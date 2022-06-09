use std::{
    cell::{self, Cell},
    collections::HashSet,
    fs,
    time::Duration,
};

use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Rect, ttf::FontStyle,
};
use sudoku::{board::Board, cell::CellValue, draw_custom_rect, draw_text, get_input};

fn run() -> Result<(), String> {
    let path = "input/lehke.txt"; //get_input().or(Err("Invalid input"))?;
    let path = path.trim().to_string();
    if !path.is_ascii() || path.len() == 0 {
        return Err("Invalid input".into());
    }

    let _board = fs::read_to_string(path).or(Err("Failed to read file"))?;

    let cells: Vec<Vec<char>> = _board.lines().map(|l| l.chars().collect()).collect();
    let mut board = Board::new(cells)?;
    board.is_solved();
    println!("{:?}", board);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    const WIDTH: i32 = 600;
    const HEIGHT: i32 = 700;
    const FRAMERATE: i32 = 60;

    // Window setup
    let window = video_subsystem
        .window("Sudoku", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .or(Err("Failed to initialize window"))?;

    let mut canvas = window
        .into_canvas()
        .build()
        .or(Err("Failed to aquire canvas"))?;

    let mut event_pump = sdl_context.event_pump()?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut big_font = ttf_context.load_font("assets/arial.ttf", 25).unwrap();
    big_font.set_style(FontStyle::BOLD);

    let mut small_font = ttf_context.load_font("assets/arial.ttf", 15).unwrap();
    small_font.set_style(FontStyle::ITALIC);

    // (x, y, is_doubleclicked)
    let mut highlighted_cell: Option<(u8, u8, bool)> = None;

    'main_loop: loop {
        let (w, h) = canvas.window().size();

        const BOARD_PADDING: i32 = 10;
        const CELL_PADDING: i32 = 2;
        const HINT_PADDING: i32 = 4;

        let cell_size = ((w as i32 - 4 * BOARD_PADDING) - 4 * CELL_PADDING) / 9;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace | Keycode::Delete),
                    ..
                } => {
                    if let Some(hc) = highlighted_cell {
                        let cell = board
                            .cells
                            .get_mut(hc.1 as usize)
                            .unwrap()
                            .get_mut(hc.0 as usize)
                            .unwrap();

                        if !cell.is_original {
                            if let CellValue::Number(_) = cell.value {
                                cell.value = CellValue::Empty;
                            }
                        }
                    }
                }
                Event::TextInput { text, .. } => {
                    if let Some(hc) = highlighted_cell {
                        let ch = text.chars().next().unwrap();

                        if let '1'..='9' = ch {
                            let digit = ch.to_digit(10).unwrap();
                            let cell = board
                                .cells
                                .get_mut(hc.1 as usize)
                                .unwrap()
                                .get_mut(hc.0 as usize)
                                .unwrap();
                            if !cell.is_original {
                                if hc.2 {
                                    cell.value = CellValue::Number(digit as u8);

                                    let is_solved = board.is_solved();
                                    if is_solved {
                                        println!("Congratulations, the puzzle is solved!");
                                    }
                                } else {
                                    cell.add_hint(digit as u8);
                                }
                            }
                        }
                    }
                }
                Event::MouseButtonUp { x, y, .. } => {
                    let (mut lx, mut ly) = (BOARD_PADDING, 100 + BOARD_PADDING);
                    let prev_hc = highlighted_cell;
                    highlighted_cell = None;

                    'outer: for i in 0..9 {
                        for j in 0..9 {
                            if (x >= lx && x < lx + cell_size) && (y >= ly && y < ly + cell_size) {
                                if let Some(mut hc) = prev_hc {
                                    if j == hc.0 && i == hc.1 {
                                        if !hc.2 {
                                            hc.2 = true;
                                            highlighted_cell = Some(hc);
                                        }
                                        break 'outer;
                                    }
                                }
                                if !board
                                    .cells
                                    .get_mut(i as usize)
                                    .unwrap()
                                    .get_mut(j as usize)
                                    .unwrap()
                                    .is_original
                                {
                                    highlighted_cell = Some((j, i, false));
                                    break 'outer;
                                }
                            }
                            if (j + 1) % 3 == 0 {
                                lx += BOARD_PADDING;
                            } else {
                                lx += CELL_PADDING;
                            }
                            lx += cell_size;
                        }

                        lx = BOARD_PADDING;
                        if (i + 1) % 3 == 0 {
                            ly += BOARD_PADDING;
                        } else {
                            ly += CELL_PADDING;
                        }
                        ly += cell_size;
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Render the board

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(Rect::new(0, 100, w, h - 100))?;
        let (mut x, mut y) = (BOARD_PADDING, 100 + BOARD_PADDING);

        for (i, row) in board.cells.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                canvas.set_draw_color(Color::WHITE);
                canvas
                    .fill_rect(Rect::new(x, y, cell_size as u32, cell_size as u32))
                    .unwrap();
                if let Some(hc) = highlighted_cell {
                    if j as u8 == hc.0 && i as u8 == hc.1 {
                        canvas.set_draw_color(Color::RGB(0, 120, 250));
                        let color = if !hc.2 {
                            Color::RGB(150, 150, 150)
                        } else {
                            Color::RGB(0, 120, 250)
                        };

                        canvas.set_draw_color(color);
                        draw_custom_rect(&mut canvas, x, y, cell_size as u32, cell_size as u32, 5)?;
                    }
                }

                if let CellValue::Number(n) = c.value {
                    let char_size = big_font
                        .size_of_char(char::from_digit(n as u32, 10).unwrap())
                        .unwrap();

                    let color = if c.is_original {
                        Color::BLACK
                    } else if c.is_invalid {
                        Color::RED
                    } else {
                        Color::BLUE
                    };
                    draw_text(
                        &mut canvas,
                        &big_font,
                        &n.to_string(),
                        color,
                        x + ((cell_size / 2) - (char_size.0 / 2) as i32),
                        y + ((cell_size / 2) - (char_size.1 / 2) as i32),
                        None,
                    )
                    .unwrap();
                } else {
                    if c.hints.len() > 0 {
                        let mut hint_text = String::new();
                        for (i, hint) in c.hints.iter().enumerate() {
                            hint_text.push(char::from_digit(*hint as u32, 10).unwrap());
                            if i != c.hints.len() - 1 {
                                hint_text.push(',');
                            }
                        }
                        draw_text(
                            &mut canvas,
                            &small_font,
                            &hint_text,
                            Color::RGB(100, 100, 100),
                            x + HINT_PADDING,
                            y + HINT_PADDING,
                            Some((cell_size - 2 * HINT_PADDING) as u32),
                        )?;
                    }
                }

                if (j + 1) % 3 == 0 {
                    x += BOARD_PADDING;
                } else {
                    x += CELL_PADDING;
                }
                x += cell_size;
            }
            x = BOARD_PADDING;
            if (i + 1) % 3 == 0 {
                y += BOARD_PADDING;
            } else {
                y += CELL_PADDING;
            }
            y += cell_size;
        }
        canvas.present();
        std::thread::sleep(Duration::from_millis(1000 / FRAMERATE as u64));
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprint!("{}", e);
        return;
    }
}
