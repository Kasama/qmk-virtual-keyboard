mod keyboard;

use std::time::Duration;

use clap::Parser;
use clap_num::maybe_hex;
use log::{debug, error, info};

use self::keyboard::{HidInfo, Keyboard, KeyboardResponse, Operation};

const VENDOR_ID: u16 = 0x4b41; // Kasama (unofficial)
const PRODUCT_ID: u16 = 0x564b; // Virtual Keyboard
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
        let response = keyboard.send_message(Operation::UpdateMatrix(true, 0, 0))?;
        info!("⌨: {response:?}");
        std::thread::sleep(Duration::from_millis(100));
        let response = keyboard.send_message(Operation::GetLayer)?;
        info!("⌨: {response:?}");
        let response = keyboard.send_message(Operation::UpdateMatrix(true, 2, 2))?;
        info!("⌨: {response:?}");
        let response = keyboard.send_message(Operation::UpdateMatrix(false, 2, 2))?;
        info!("⌨: {response:?}");
        std::thread::sleep(Duration::from_millis(100));
        let response = keyboard.send_message(Operation::UpdateMatrix(false, 0, 0))?;
        info!("⌨: {response:?}");

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
}
