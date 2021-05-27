#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use cosmian_std::prelude::scale_std::slice::Slice;
use cosmian_std::scale::{self, println, ClearModp, SecretModp};
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
        let (mut id_s_modp_0, mut id_s_64_0) = match pr0.read_next_record() {
            Some(res) => {
                let id_s_modp = *res.get_unchecked(0);
                let id_s_64 = SecretInteger::<64>::from(id_s_modp);
                (id_s_modp, id_s_64)
            }
            None => {
                break 'global;
            }
        };

        // First fetch id of participant 1
        let (mut id_s_modp_1, mut id_s_64_1) = match pr1.read_next_record() {
            Some(res) => {
                let id_s_modp = *res.get_unchecked(0);
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
                match pr0.read_next_record() {
                    Some(res) => {
                        id_s_modp_0 = *res.get_unchecked(0);
                        id_s_64_0 = SecretInteger::<64>::from(id_s_modp_0);
                    }
                    None => {
                        break 'global;
                    }
                };
            } else {
                println!(" -> ID 0 > ID 1");
                match pr1.read_next_record() {
                    Some(res) => {
                        id_s_modp_1 = *res.get_unchecked(0);
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
    nb_records: u64,
    current_record: u64,
    eof: bool,
}

impl<const P: u32> PlayerReader<P> {
    pub fn new(player: Player<P>) -> PlayerReader<P> {
        PlayerReader {
            player,
            nb_records: 0,
            current_record: 0,
            eof: false,
        }
    }

    pub fn read_next_record(&mut self) -> Option<Slice<SecretModp>> {
        if self.eof {
            return None;
        }
        if self.current_record >= self.nb_records {
            let nb_records: ClearModp =
                SecretModp::private_input(self.player, Channel::<0>).reveal();
            self.nb_records = i64::from(nb_records) as u64;
            if self.nb_records == 0 {
                self.eof = true;
                return None;
            }
            self.current_record = 0;
        }

        let num_cols = SecretModp::private_input(self.player, Channel::<1>).reveal();
        let num_cols = i64::from(num_cols) as u64;

        let mut row = Slice::uninitialized(num_cols);

        for row_nb in 0..num_cols {
            row.set(
                row_nb,
                &SecretModp::private_input(self.player, Channel::<2>),
            );
        }

        self.current_record += 1;

        Some(row)
    }
}
