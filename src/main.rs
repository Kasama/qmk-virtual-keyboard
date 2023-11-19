mod event_input_device;
mod key_event;
mod keyboard;
mod uinput;

use std::convert::Into;
use std::io::Write;

use clap::Parser;
use clap_num::maybe_hex;
use log::error;

use key_event::KeyEvent;

use crate::key_event::key_code::KeyCode;
use crate::key_event::matix_mapper::{Layout, LayoutItem, MatrixPosition};

use self::event_input_device::EventDevice;
use self::keyboard::{HidInfo, Keyboard, KeyboardResponse, Operation};

const VENDOR_ID: u16 = 0x4b41; // Kasama (unofficial)
                               // const PRODUCT_ID: u16 = 0x564b; // Virtual Keyboard
const PRODUCT_ID: u16 = 0x504d; // Macropad
const USAGE_PAGE: u16 = 0xff60; // QMK
const USAGE: u16 = 0x61; // QMK

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct App {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, action = clap::ArgAction::Count)]
    /// Increases log verbosity on each appearance. -vvv will print out trace logs
    verbose: u8,

    #[arg(short, long)]
    /// Suppress all output when set
    quiet: bool,

    #[arg(long, default_value_t = VENDOR_ID, value_parser=maybe_hex::<u16>)]
    /// HID Vendor ID
    vid: u16,
    #[arg(long, default_value_t = PRODUCT_ID, value_parser=maybe_hex::<u16>)]
    /// HID Product ID
    pid: u16,
    #[arg(short = 'p', long, default_value_t = USAGE_PAGE, value_parser=maybe_hex::<u16>)]
    /// HID Usage Page
    usage_page: u16,
    #[arg(short, long, default_value_t = USAGE, value_parser=maybe_hex::<u16>)]
    /// HID Usage
    usage: u16,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    PrintKeyboardLayer,
    KeyboardBootloader,
    ChangeKeyboardLayer { layer: u8 },
    SendKey { row: u8, col: u8 },
    VirtualKeyboard { device: String, config: String },
    GenerateMatrixMap { device: String, rows: u8, cols: u8 },
}

fn print_error<T, E: std::fmt::Debug>(r: Result<T, E>) {
    r.map(|_| ()).unwrap_or_else(|e| error!("Error: {:?}", e));
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    let app = App::parse();

    if !app.quiet {
        simple_logger::init_with_level(match app.verbose {
            0 => log::Level::Error,
            1 => log::Level::Info,
            2 => log::Level::Debug,
            3.. => log::Level::Trace,
        })?;
    }

    match app.command {
        Commands::PrintKeyboardLayer => print_error(app.print_keyboard_layer()),
        Commands::KeyboardBootloader => print_error(app.keyboard_bootloader()),
        Commands::ChangeKeyboardLayer { layer } => print_error(app.change_keyboard_layer(layer)),
        Commands::SendKey { row, col } => print_error(app.tap_key(row, col)),
        Commands::VirtualKeyboard {
            ref device,
            ref config,
        } => print_error(app.virtual_keyboard(device, config)),
        Commands::GenerateMatrixMap {
            ref device,
            rows,
            cols,
        } => print_error(app.generate_matrix_map(device, rows, cols)),
    };

    Ok(())
}

impl App {
    fn connect_to_keyboard(&self) -> Result<Keyboard, anyhow::Error> {
        Keyboard::new(&HidInfo {
            vendor_id: self.vid,
            product_id: self.pid,
            usage_page: self.usage_page,
            usage: self.usage,
        })
    }

    fn virtual_keyboard(&self, device: &str, matrix_config_file: &str) -> anyhow::Result<()> {
        let device = EventDevice::from_path(device)?;
        let keyboard = self.connect_to_keyboard()?;

        let matrix_file_contents = std::fs::read_to_string(matrix_config_file)?;
        let matrix: Layout = serde_json::from_str(&matrix_file_contents)?;

        let mut matrix_mapper: [MatrixPosition; u8::MAX as usize] =
            [(0, 0).into(); u8::MAX as usize];

        matrix.layout.iter().for_each(|layout_item| {
            let keycode: Result<KeyCode, _> = layout_item.label.parse();
            if let Ok(keycode) = keycode {
                matrix_mapper[Into::<usize>::into(keycode)] = layout_item.matrix
            };
        });

        loop {
            let event = device.next()?;

            if let KeyEvent::Press(code, held, _) = event {
                if !held && code != KeyCode::NONE {
                    let matrix_pos = matrix_mapper[code as usize];
                    let _response = keyboard.send_message(Operation::UpdateMatrix(
                        true,
                        matrix_pos.row,
                        matrix_pos.col,
                    ))?;
                }
            }

            if let KeyEvent::Release(code, _) = event {
                let matrix_pos = matrix_mapper[code as usize];
                let _response = keyboard.send_message(Operation::UpdateMatrix(
                    false,
                    matrix_pos.row,
                    matrix_pos.col,
                ))?;
            }
        }
    }

    fn tap_key(&self, _row: u8, _col: u8) -> Result<(), anyhow::Error> {
        let keyboard = self.connect_to_keyboard()?;

        // let response = keyboard.send_message(Operation::UpdateMatrix(true, 3, 3))?;
        // println!("⌨: {response:?}");
        // std::thread::sleep(Duration::from_millis(100));
        // let response = keyboard.send_message(Operation::UpdateMatrix(true, 2, 2))?;
        // println!("⌨: {response:?}");
        // let response = keyboard.send_message(Operation::UpdateMatrix(false, 2, 2))?;
        // println!("⌨: {response:?}");
        // std::thread::sleep(Duration::from_millis(100));
        // let response = keyboard.send_message(Operation::UpdateMatrix(false, 3, 3))?;
        // println!("⌨: {response:?}");
        // let _response = keyboard.send_message(Operation::ChangeLayer(0))?;
        // std::thread::sleep(std::time::Duration::from_millis(100));
        let _response = keyboard.send_message(Operation::UpdateMatrix(true, 0, 0))?;
        let _response = keyboard.send_message(Operation::UpdateMatrix(false, 0, 0))?;

        Ok(())
    }

    fn print_keyboard_layer(&self) -> Result<(), anyhow::Error> {
        let keyboard = self.connect_to_keyboard()?;

        let response = keyboard.send_message(Operation::GetLayer)?;

        if let KeyboardResponse::CurrentLayer(layer) = response {
            println!("⌨: {}", keyboard::Layers::from(layer).to_string());
        }

        Ok(())
    }

    fn change_keyboard_layer(&self, layer: u8) -> Result<(), anyhow::Error> {
        let keyboard = self.connect_to_keyboard()?;

        let response = keyboard.send_message(Operation::ChangeLayer(layer))?;

        if let KeyboardResponse::CurrentLayer(layer) = response {
            println!("Current layer: {}", layer);
        }

        Ok(())
    }

    fn keyboard_bootloader(&self) -> Result<(), anyhow::Error> {
        let keyboard = self.connect_to_keyboard()?;

        let _response = keyboard.send_message(Operation::Bootloader)?;

        Ok(())
    }

    fn generate_matrix_map(&self, device: &str, rows: u8, cols: u8) -> Result<(), anyhow::Error> {
        let device = EventDevice::from_path(device)?;

        eprintln!("Press one button on the keyboard at a time from left to right and top to bottom to generate the matrix map. Press Ctrl+C to exit.");
        eprintln!("The first will be position (0, 0). Press that button again on any position to skip it if there is no button there.\n");

        let mut skip = KeyCode::NONE;

        let result_matrix: Vec<LayoutItem> = (0..rows)
            .map(|r| {
                (0..cols)
                    .map(|c| {
                        eprint!("Press {r},{c}: ");
                        std::io::stderr().flush()?;
                        let code = loop {
                            let event = device.next()?;

                            if let KeyEvent::Press(code, false, _) = event {
                                if code == skip {
                                    break KeyCode::NONE;
                                }

                                if r == 0 && c == 0 {
                                    skip = code;
                                }

                                break code;
                            }
                        };

                        eprintln!("{:?}", code);
                        match code {
                            KeyCode::NONE => Ok(None),
                            _ => Ok(Some(LayoutItem {
                                matrix: MatrixPosition { row: r, col: c },
                                x: c,
                                y: r,
                                label: format!("{:?}", code),
                            })),
                        }
                    })
                    .collect::<anyhow::Result<Vec<_>>>()
                    .map(|v| v.into_iter().flatten().collect::<Vec<_>>())
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map(|v| v.into_iter().flatten().collect())?;

        let layout = Layout {
            layout: result_matrix,
        };

        let serialized_str = serde_json::to_string(&layout)?;
        println!("{serialized_str}");

        Ok(())
    }
}
