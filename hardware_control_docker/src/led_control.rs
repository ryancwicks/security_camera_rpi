//! This file contains the code for controlling the LED outputs.
use tokio::sync::mpsc;
use crate::SystemState;


pub async fn led_task (mut led_rx: mpsc::Receiver<SystemState>) {
    let mut led_state = SystemState::Idle;

    let mut led_cycle_count = 0;
    loop {
        match tokio::time::timeout(std::time::Duration::from_millis(500), led_rx.recv()).await {
            Ok(val) => {
                match val {
                    Some(msg) => {
                        led_state = msg;
                    },
                    None => ()
                }

            },
            Err(_) => { //timer expired, no message recieved, run the blink logic/LED state machine.
                led_cycle_count += 1;
                match &led_state {
                    SystemState::Idle => {
                        //LED off, do nothing.
                    },
                    SystemState::Recording => {
                        if led_cycle_count == 2 {
                            //toggle the led state
                        }
                    },
                    SystemState::SystemError(_) => {
                        if led_cycle_count == 1 {
                            //LED high
                        } else {
                            //LED low
                        }

                    },
                }
                if led_cycle_count >= 2 {
                    led_cycle_count = 0;
                }
            }
        };
        
    }
}