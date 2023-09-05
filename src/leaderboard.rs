use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::plasma::*;
use curl::easy::*;
use serde::Deserialize;
use std::io::Write;
use std::{thread, time};
use termion::event::Key;
use termion::input::Keys;
use termion::raw::RawTerminal;
use termion::terminal_size;
use termion::AsyncReader;

pub fn leaderboard(stdin: &mut Keys<AsyncReader>, stdout: &mut RawTerminal<std::io::Stdout>) {
    print!("{}", termion::cursor::Goto(1, 1));
    print!("{}", termion::clear::All);

    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new(term_size);

    let frame_fps = 20;
    let frame_len = time::Duration::from_micros(1000000 / frame_fps);
    let mut delta_time: f32 = frame_len.as_micros() as f32 / 1000000.0;

    let mut plasma = FireDrawer::new(term_size);

    let players = get_leaderboard();
    let players_displayed: usize = 15;
    let mut players_offset: i8 = 0;

    loop {
        let frame_start = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Ctrl('c') | Key::Esc | Key::Char('q') | Key::Char('\n') => {
                        break;
                    }
                    Key::Up => players_offset = (players_offset - 1).max(0),
                    Key::Down => {
                        players_offset = (players_offset + 1)
                            .min(players.list.len() as i8 - players_displayed as i8)
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        plasma.update(delta_time);

        scr.flush_text_entries();

        for i in 0..players_displayed {
            let message_y = 3;
            let message = String::from("                                              ");
            let message_x = (term_size.0 - message.len() as u16) / 2;
            scr.add_text_entry(&TextEntry {
                position: (message_x as f32, message_y as f32 + i as f32 * 2.0),
                string: message,
                color_palette: TextColorPalette::Text,
            });
        }

        for i in 0..players_displayed {
            let pos = (i + players_offset as usize) as usize;
            let message_y = 3;
            let message = format!(
                " {}. - {} : {} ",
                pos + 1,
                players.list[pos].name.clone(),
                players.list[pos].score,
            );
            let message_x = (term_size.0 - message.len() as u16) / 2;
            scr.add_text_entry(&TextEntry {
                position: (message_x as f32, message_y as f32 + i as f32 * 2.0),
                string: message,
                color_palette: match pos {
                    0..=3 => TextColorPalette::Warning,
                    4..=10 => TextColorPalette::Menu,
                    _ => TextColorPalette::Text,
                },
            });
        }

        scr.display_text();

        plasma.draw();

        stdout.flush().unwrap();

        if let Some(i) = (frame_len).checked_sub(frame_start.elapsed()) {
            thread::sleep(i)
        }

        delta_time =
            time::Instant::now().duration_since(frame_start).as_micros() as f32 / 1000000.0;
    }
}

#[derive(Debug, Deserialize)]
struct LeaderboardEntry {
    name: String,
    score: u32,
}

#[derive(Debug, Deserialize)]
struct LeaderboardStruct {
    list: Vec<LeaderboardEntry>,
}

fn get_leaderboard() -> LeaderboardStruct {
    //hey there cowboy, please don't ruin the fun for others :)
    //happy you are curious, hit me up on Twitter @c64cosmin :D
    let html_leader: String = do_http_request(String::from(
        "https://www.stupidrat.com/terminoids/hi/score.php?mode=get",
    ));

    let parsed_data: Result<LeaderboardStruct, serde_json::Error> =
        serde_json::from_str(&html_leader);

    match parsed_data {
        Ok(data) => {
            return data;
        }
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
        }
    }

    return LeaderboardStruct { list: Vec::new() };
}

fn do_http_request(url: String) -> String {
    let mut easy = Easy::new();
    easy.url(url.as_str()).unwrap();

    let mut response_data = Vec::new();

    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    easy.perform().unwrap();

    match easy.response_code() {
        Ok(200) => {
            return String::from_utf8_lossy(&response_data).to_string();
        }
        Ok(status_code) => {
            eprintln!("Request failed with status code: {}", status_code);
        }
        Err(err) => {
            eprintln!("Request failed with error: {}", err);
        }
    }
    String::from("")
}
