#![no_std]
#![no_main]

use embedded_sdmmc::{Directory, Volume, VolumeIdx};

use longan_nano::sdcard::SdCard;
use panic_halt as _;

use riscv_rt::entry;
use longan_nano::hal::{pac, prelude::*};
use longan_nano::{sdcard, sprint, sprintln};

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
    let mut sdcard = sdcard::configure(dp.SPI1, gpiob, &mut rcu);

    sprint!("Initializing SD card ... ");
    if let Err(_) = sdcard.device().init() {
        sprintln!("Failed to initialize sdcard.");
    } else {
        sprintln!("OK");
        sprint!("SD Card Capacity: ");
        if let Ok(size) = sdcard.device().card_size_bytes() {
            sprintln!("{} MB", size / 1000 / 1000);
            // open the first partition
            sprintln!("Partition 0:");
            if let Ok(mut volume) = sdcard.get_volume(VolumeIdx(0)) {
                // list files in root dir
                if let Ok(root_dir) = sdcard.open_root_dir(&volume) {
                    if let Err(_) = sdcard.iterate_dir(&volume, &root_dir, | entry | {
                        sprintln!("{: >5}B  {}", entry.size, entry.name);
                    }) {
                        sprintln!("Error while iterating over files.");
                    }
                    if let Ok(_) = sdcard.find_directory_entry(&volume, &root_dir, "W_TST.TXT") {
                        // if a file with the name W_TST.TXT is present, do a write test
                        write_test(&mut sdcard, &mut volume, &root_dir);
                    }
                } else {
                    sprintln!("Could not open root directory.");
                }
            } else {
                sprintln!("Could not open partition 0.");
            }
        } else {
            sprintln!("Failed to read card size.");
        }
    }

    sprintln!("done.");
    loop { }
}

fn write_test(sdcard: &mut SdCard, volume: &mut Volume, dir: &Directory) {
    sprint!("Write test: ");
    if let Ok(mut file) = sdcard.open_file_in_dir(volume, dir, "W_TST.CSV", embedded_sdmmc::Mode::ReadWriteCreateOrTruncate) {
        let data = "1,2,3,4,20";
        if let Ok(size_written) = sdcard.write(volume, &mut file, data.as_bytes()) {
            sprintln!("Success (Wrote {}B)", size_written);
        } else {
            sprintln!("Failed");
        }
        if let Err(_) = sdcard.close_file(volume, file) {
            sprintln!("Could not close file.");
        }
    } else {
        sprintln!("Could not open file for writing.");
    }
}
