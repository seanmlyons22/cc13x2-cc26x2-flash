#![no_std]
#![no_main]

use cc13x2_cc26x2_pac as cc13x2_cc26x2;
use flash_algorithm::*;
use rtt_target::{rprintln, rtt_init_print};

extern "C" {
    fn NOROM_FlashSectorErase(ui32SectorAddress: u32) -> u32;
    fn NOROM_FlashProgram(pui8DataBuffer: *const u8, ui32Address: u32, ui32Count: u32) -> u32;
}

struct Algorithm;

algorithm!(Algorithm, {
    flash_address: 0x0,
    flash_size: 360448,
    page_size: 8192,
    empty_value: 0xFF,
    sectors: [{
        size: 8192,
        address: 0x0,
    }]
});
const FAPI_STATUS_SUCCESS: u32 = 0x00000000; // Function completed successfully

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        rtt_init_print!();
        rprintln!("Init");
        let p = cc13x2_cc26x2::Peripherals::take().unwrap();

        // Setup PRCM, power the perpipheral and serial domains
        p.PRCM.pdctl0periph.write(|w| w.on().set_bit());
        p.PRCM.pdctl0serial.write(|w| w.on().set_bit());
        p.PRCM.gpioclkgr.write(|w| w.clk_en().set_bit());
        p.PRCM.clkloadctl.write(|w| w.load().set_bit());

        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");
        let flash_size = FlashDevice.device_size;
        let page_size = FlashDevice.page_size;
        let num_pages = flash_size / page_size;
        for page in 0..num_pages {
            let addr = page * page_size;
            self.erase_sector(addr)?;
        }
        Ok(())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector addr:{}", addr);

        let status: u32 = unsafe { NOROM_FlashSectorErase(addr) };
        match status {
            FAPI_STATUS_SUCCESS => Ok(()),
            _ => Err(ErrorCode::new(status).unwrap()),
        }
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Program Page addr:{} size:{}", addr, data.len());
        let status: u32 =
            unsafe { NOROM_FlashProgram(data.as_ptr(), addr, data.len().try_into().unwrap()) };
        match status {
            FAPI_STATUS_SUCCESS => Ok(()),
            _ => Err(ErrorCode::new(status).unwrap()),
        }
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        rprintln!("Deinit");
    }
}
