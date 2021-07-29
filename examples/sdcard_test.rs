#![no_std]
#![no_main]

use embedded_sdmmc::{Directory, Volume, VolumeIdx};

use longan_nano::sdcard::SdCard;
use panic_halt as _;

use riscv_rt::entry;
use longan_nano::hal::{pac, prelude::*};
use longan_nano::{sdcard, sdcard_pins, sprint, sprintln};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp.RCU.configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    longan_nano::stdout::configure(dp.USART0, gpioa.pa9, gpioa.pa10, 115_200.bps(), &mut afio, &mut rcu);

    let gpiob = dp.GPIOB.split(&mut rcu);
    let sdcard_pins = sdcard_pins!(gpiob);
    let mut sdcard = sdcard::configure(dp.SPI1, sdcard_pins, sdcard::SdCardFreq::Safe, &mut rcu);

    sprint!("Initializing SD card ... ");
    if let Err(_) = sdcard.device().init() {
        sprintln!("Failed to initialize sdcard.");
    } else {
        sprintln!("OK");

        let size = sdcard.device().card_size_bytes().unwrap();
        sprintln!("SD Card Capacity: {} MB", size / 1000 / 1000);

        // open the first partition
        sprintln!("Partition 0:");
        let mut volume = sdcard.get_volume(VolumeIdx(0)).unwrap();

        // list files in root dir
        let root_dir = sdcard.open_root_dir(&volume).unwrap();
        sdcard.iterate_dir(&volume, &root_dir, | entry | {
            sprintln!("{: >5}B  {}", entry.size, entry.name);
        }).unwrap();

        // if a file with the name SDTST.TXT is present, do a read/write test
        if let Ok(_) = sdcard.find_directory_entry(&volume, &root_dir, "SDTST.TXT") {
            read_write_test(&mut sdcard, &mut volume, &root_dir);
        }
    }
    sprintln!("Done");

    loop { }
}

fn read_write_test(sdcard: &mut SdCard, volume: &mut Volume, dir: &Directory) {
    sprint!("Write test: ");
    let mut file = sdcard.open_file_in_dir(volume, dir, "SDTST.CSV", embedded_sdmmc::Mode::ReadWriteCreateOrTruncate).unwrap();
    let data = "1,2,3,4,20";
    if let Ok(size_written) = sdcard.write(volume, &mut file, data.as_bytes()) {
        sprintln!("Success (Wrote {} bytes)", size_written);
    } else {
        sprintln!("Failed");
    }
    sdcard.close_file(volume, file).unwrap();

    sprint!("Read test: ");
    let mut file = sdcard.open_file_in_dir(volume, dir, "SDTST.CSV", embedded_sdmmc::Mode::ReadOnly).unwrap();
    let mut buffer: [u8; 32] = [0; 32];
    if let Ok(size_read) = sdcard.read(volume, &mut file, &mut buffer) {
        if size_read == data.len() && buffer[0..size_read].eq(data.as_bytes()) {
            sprintln!("Success (Read same {} bytes)", size_read);
        } else {
            sprintln!("Content differs.");
        }
    } else {
        sprintln!("Failed");
    }
    sdcard.close_file(volume, file).unwrap();
}
