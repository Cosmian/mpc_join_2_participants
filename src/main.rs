#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use cosmian_std::prelude::scale_std::slice::Slice;
use cosmian_std::scale::{self, println, Channel, ClearModp, SecretModp};
use cosmian_std::{prelude::*, OutputRow};

// Players
// PARTICIPANT_2 is just an arbiter, as MPC needs at least 3 participants
const PARTICIPANT_0: Player<0> = Player::<0>; // data provider, receives results
const PARTICIPANT_1: Player<1> = Player::<1>; // data provider, receives results

#[inline(never)]
#[scale::main(KAPPA = 40)]
fn main() {
    println!("##### Reading from participants");

    let mut pr0 = PlayerReader::new(PARTICIPANT_0);
    let mut pr1 = PlayerReader::new(PARTICIPANT_1);

    'global: loop {
        // First fetch id of participant 0
        let (mut id_s_modp_0, mut id_s_64_0) = match pr0.read_next_value() {
            Some(id_s_modp) => {
                let id_s_64 = SecretInteger::<64>::from(id_s_modp);
                (id_s_modp, id_s_64)
            }
            None => {
                break 'global;
            }
        };

        // First fetch id of participant 1
        let (mut id_s_modp_1, mut id_s_64_1) = match pr0.read_next_value() {
            Some(id_s_modp) => {
                let id_s_64 = SecretInteger::<64>::from(id_s_modp);
                (id_s_modp, id_s_64)
            }
            None => {
                break 'global;
            }
        };

        loop {
            if i64::from(id_s_64_0.eq(id_s_64_1).reveal()) == 1 {
                println!(" -> match");
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
                println!(" -> ID 0 < ID 1");
                // Fetch next id_participant 0
                match pr0.read_next_value() {
                    Some(res) => {
                        id_s_modp_0 = res;
                        id_s_64_0 = SecretInteger::<64>::from(id_s_modp_0);
                    }
                    None => {
                        break 'global;
                    }
                };
            } else {
                println!(" -> ID 0 > ID 1");
                match pr1.read_next_value() {
                    Some(res) => {
                        id_s_modp_1 = res;
                        id_s_64_1 = SecretInteger::<64>::from(id_s_modp_1);
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

struct PlayerReader<const P: u32> {
    player: Player<P>,
    nb_cols: u64,
    current_col: u64,
    values: Slice<SecretModp>,
    current_index: u64,
    eof: bool,
}

impl<const P: u32> PlayerReader<P> {
    pub fn new(player: Player<P>) -> PlayerReader<P> {
        PlayerReader {
            player,
            nb_cols: 0,
            current_col: 0,
            eof: false,
            values: Slice::uninitialized(0),
            current_index: 0,
        }
    }

    pub fn read_next_value(&mut self) -> Option<SecretModp> {
        if self.eof {
            return None;
        }
        if self.current_index >= self.values.len() {
            if self.read_next_column() {
                self.current_index = 0;
            } else {
                return None;
            }
        }
        let value = *(self.values.get_unchecked(self.current_index));
        self.current_index += 1;
        Some(value)
    }

    pub fn read_next_column(&mut self) -> bool {
        if self.current_col >= self.nb_cols {
            if !self.read_next_record() {
                return false;
            }
            self.current_col = 0;
        }

        let num_values = SecretModp::private_input(self.player, Channel::<1>).reveal();
        let num_values = i64::from(num_values) as u64;
        if num_values == 0 {
            self.eof = true;
            return false;
        }

        self.values = Slice::private_input(num_values, self.player, Channel::<2>);

        true
    }

    pub fn read_next_record(&mut self) -> bool {
        let nb_cols: ClearModp = SecretModp::private_input(self.player, Channel::<0>).reveal();
        self.nb_cols = i64::from(nb_cols) as u64;
        if self.nb_cols == 0 {
            self.eof = true;
            return false;
        }
        true
    }
}
