#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use cosmian_std::scale::{self, println};
use cosmian_std::{prelude::*, Column, InputRow, OutputRow};

// Players
// PARTICIPANT_2 is just an arbiter, as MPC needs at least 3 participants
const PARTICIPANT_0: Player<0> = Player::<0>; // data provider, receives results
const PARTICIPANT_1: Player<1> = Player::<1>; // data provider, receives results

#[inline(never)]
#[scale::main(KAPPA = 40)]
fn main() {
    println!("##### Reading from participants");

    'global: loop {
        // First fetch id of participant 0
        let (mut id_s_modp_0, mut id_s_64_0) = match read_next_id(PARTICIPANT_0) {
            Some(res) => res,
            None => {
                break 'global;
            }
        };

        // First fetch id of participant 1
        let (mut id_s_modp_1, mut id_s_64_1) = match read_next_id(PARTICIPANT_1) {
            Some(res) => res,
            None => {
                break 'global;
            }
        };

        loop {
            if i64::from(id_s_64_0.eq(id_s_64_1).reveal()) == 1 {
                // println!(" -> match");
                // Create the next row we are going to output to the data consumer
                let mut output_row_0 = OutputRow::new(PARTICIPANT_0);
                // Create the next row we are going to output to the data consumer
                let mut output_row_1 = OutputRow::new(PARTICIPANT_1);

                // Send id to both participant 0 and participant 1
                output_row_0.append(id_s_modp_0);
                output_row_1.append(id_s_modp_1);

                // the rows will be automatically flushed to the participants
                // this break returns to the global loop and fetches bth IDs
                break;
            } else if i64::from(id_s_64_0.lt(id_s_64_1).reveal()) == 1 {
                // println!(" -> ID 0 < ID 1");
                // Fetch next id_participant 0
                match read_next_id(PARTICIPANT_0) {
                    Some((id_s_modp, id_s_64)) => {
                        id_s_modp_0 = id_s_modp;
                        id_s_64_0 = id_s_64
                    }
                    None => {
                        break 'global;
                    }
                };
            } else {
                // println!(" -> ID 0 > ID 1");
                match read_next_id(PARTICIPANT_1) {
                    Some((id_s_modp, id_s_64)) => {
                        id_s_modp_1 = id_s_modp;
                        id_s_64_1 = id_s_64
                    }
                    None => {
                        break 'global;
                    }
                };
            }
        }
    }
    println!("##### End of processing");
}

#[inline(always)]
fn read_next_id<const P: u32>(player: Player<P>) -> Option<(SecretModp, SecretInteger<64>)> {
    let mut row_participant = InputRow::read(player);
    let id_s_modp = match row_participant.next_col() {
        Some(Column::SecretModp(id)) => id,
        None => {
            println!("    <-- no more data for participant ", P);
            return None;
        }
        _ => {
            scale::panic!("bad data format for data from participant ", P);
            return None;
        }
    };
    // println!(" <- read ID ", P);
    // secret comparisons are performed on 64 bit integers
    let id_s_i64 = SecretInteger::<64>::from(id_s_modp);
    // println!("- converted ID ", P);
    Some((id_s_modp, id_s_i64))
}
