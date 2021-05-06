#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use cosmian_std::scale::{self, println};
use cosmian_std::{prelude::*, Column, InputRow, OutputRow};

// Players
// PARTICIPANT_2 is just an arbiter, as MPC needs at least 3 participants
const PARTICIPANT_0: u32 = 0; // data provider, receives results
const PARTICIPANT_1: u32 = 1; // data provider, receives results

#[inline(never)]
#[scale::main(KAPPA = 40)]
fn main() {
    println!("##### Reading from players");

    'global: loop {
        // First fetch of id participant 0
        let mut row_participant_0 = InputRow::read(Player::<PARTICIPANT_0>);
        // The id is a `SecretModp`: its value never appears in clear text in the program memory
        let mut id_participant_0 = match row_participant_0.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                println!("    <-- no more data for participant 0");
                break 'global;
            }
            _ => {
                scale::panic!("bad data format for data from participant 0!");
                return;
            }
        };
        println!(" <- read ID 0");

        // First fetch of id player 2
        let mut row_participant_1 = InputRow::read(Player::<PARTICIPANT_1>);
        // The id is a `SecretModp`: its value never appears in clear text in the program memory
        let mut id_participant_1 = match row_participant_1.next_col() {
            Some(Column::SecretModp(id)) => id,
            None => {
                println!("    <-- no more data for participant 1");
                break 'global;
            }
            _ => {
                scale::panic!("bad data format for data from participant 1!");
                return;
            }
        };
        println!(" <- read ID 1");

        loop {
            // secret comparisons are performed on 64 bit integers
            let id_0 = SecretInteger::<64>::from(id_participant_0);
            let id_1 = SecretInteger::<64>::from(id_participant_1);
            // let delta: SecretI64 = SecretI64::from(id_participant_0 - id_participant_1);
            println!("- converted IDs");
            let equality: SecretModp = id_0.eq(id_1);
            println!("- computed equality");
            let revealed = equality.reveal();
            println!("- revealed");
            let res = i64::from(revealed);
            println!("- to i64");
            if res == 1 {
                println!(" -> match");
                // Create the next row we are going to output to the data consumer
                let mut output_row_0 = OutputRow::new(Player::<PARTICIPANT_0>);
                // Create the next row we are going to output to the data consumer
                let mut output_row_1 = OutputRow::new(Player::<PARTICIPANT_1>);

                // Send id to both participant 0 and participant 1
                output_row_0.append(id_participant_0);
                output_row_1.append(id_participant_1);

                // the rows will be automatically flushed to the participants
                // this break returns to the global loop and fetches bth IDs
                break;
            } else if i64::from(id_0.lt(id_1).reveal()) == 1 {
                println!(" -> ID 0 < ID 1");
                // Fetch next id_participant 0
                let mut row_participant_0 = InputRow::read(Player::<PARTICIPANT_0>);
                id_participant_0 = match row_participant_0.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        println!("    <-- no more data for participant 0");
                        break 'global;
                    }
                    _ => {
                        scale::panic!("bad data format for data from participant 0!");
                        return;
                    }
                };
                println!(" <- read ID 0");
            } else {
                println!(" -> ID 1 < ID 0");
                // Fetch next id_participant 1
                let mut row_participant_1 = InputRow::read(Player::<PARTICIPANT_1>);
                id_participant_1 = match row_participant_1.next_col() {
                    Some(Column::SecretModp(id)) => id,
                    None => {
                        println!("    <-- no more data for participant 1");
                        break;
                    }
                    _ => {
                        scale::panic!("bad data format for data from participant 1!");
                        return;
                    }
                };
                println!(" <- read ID 1");
            }
        }
    }
    println!("##### End of processing");
}
