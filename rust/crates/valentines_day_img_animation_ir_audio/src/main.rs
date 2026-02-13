#![no_std]
#![no_main]
use fwwasm_ffi::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
     loop {}
}

#[unsafe(no_mangle)]
extern "C" fn _start() {
    unsafe {
        // Create initial panel with valentine image
        addPanel(1, 1, 0, 0, 0, 100, 0, 0, 0);  // Dark red background
        
        // Add valentine start image
        addControlPictureFromFile(
            1,
            1,
            0,
            0,
            c"picturestart.fwi".as_ptr(),
            1,
        );
        
        showPanel(1);
        
        // Clear any pending events from startup
        while hasEvent() != 0 {
            let mut dummy = [0; FW_GET_EVENT_DATA_MAX as usize];
            getEventData(dummy.as_mut_ptr());
        }
        
        // Wait for green button press to start animation
        'waiting: loop {
            // Play sound in loop
            playSoundFromNameOrID(c"DaveICantDoThat".as_ptr(), 0);
            
            // Check for button press multiple times while sound plays
            for _ in 0..20 {
                waitms(100);
                
                if hasEvent() != 0 {
                    let mut event_data = [0; FW_GET_EVENT_DATA_MAX as usize];
                    let last_event = getEventData(event_data.as_mut_ptr());
                    
                    // Check for button press (event_data[0] == 0 means pressed, not released)
                    let pressed = event_data[0] == 0;
                    
                    if !pressed {
                        continue;
                    }
                    
                    // Check if green button was pressed
                    if FWGuiEventType::FWGUI_EVENT_GREEN_BUTTON.0 == last_event as u32 {
                        // Transmit IR codes
                        sendIRData(0xa659ff00);
                        waitms(100);
                        sendIRData(0xa758ff00);
                        waitms(100);
                        sendIRData(0xba45ff00);
                        waitms(100);
                        sendIRData(0xbdfbffbf);
                        waitms(100);
                        
                        // Flash LEDs red in heartbeat pattern (double beat)
                        // First beat
                        for led in 0..7 {
                            setBoardLED(
                                led,
                                255,
                                0,
                                0,
                                200,
                                LEDManagerLEDMode::ledflash,
                            );
                        }
                        waitms(200);
                        
                        // Second beat
                        for led in 0..7 {
                            setBoardLED(
                                led,
                                255,
                                0,
                                0,
                                200,
                                LEDManagerLEDMode::ledflash,
                            );
                        }
                        waitms(500);
                        
                        // Exit to image loop with labeled break
                        break 'waiting;
                    }
                }
            }
        }
        
        // Now loop through images continuously with IR messages and audio
        loop {
            // Play the song
            playSoundFromNameOrID(c"DaveICantDoThat".as_ptr(), 0);
            
            // Transmit IR codes with delays
            sendIRData(0xa659ff00);
            waitms(300);
            sendIRData(0xa758ff00);
            waitms(300);
            sendIRData(0xba45ff00);
            waitms(300);
            sendIRData(0xbdfbffbf);
            waitms(500);
            
            // Show picture1.fwi
            addPanel(2, 1, 0, 0, 0, 0, 0, 0, 0);
            addControlPictureFromFile(2, 2, 0, 0, c"picture1.fwi".as_ptr(), 1);
            showPanel(2);
            waitms(500);
            
            // Show picture2.fwi
            addPanel(3, 1, 0, 0, 0, 0, 0, 0, 0);
            addControlPictureFromFile(3, 3, 0, 0, c"picture2.fwi".as_ptr(), 1);
            showPanel(3);
            waitms(500);
            
            // Show picture3.fwi
            addPanel(4, 1, 0, 0, 0, 0, 0, 0, 0);
            addControlPictureFromFile(4, 4, 0, 0, c"picture3.fwi".as_ptr(), 1);
            showPanel(4);
            waitms(500);
            
            // Show picture4.fwi
            addPanel(5, 1, 0, 0, 0, 0, 0, 0, 0);
            addControlPictureFromFile(5, 5, 0, 0, c"picture4.fwi".as_ptr(), 1);
            showPanel(5);
            waitms(500);
        }
    }
}
