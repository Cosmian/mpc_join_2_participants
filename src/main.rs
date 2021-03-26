#![no_std]
#![no_main]

use scale_std::slice::Slice;

// Players
// PARTICIPANT_3 is just an arbiter, as MPC needs at least 3 participants
const PARTICIPANT_1: u32 = 0; // data provider, receives results
const PARTICIPANT_2: u32 = 1; // data provider, receives results

#[inline(never)]
#[scale::main(KAPPA = 40)]
fn main() {
    print!("##### Reading from players\n");

    'global: loop {
        // First fetch of id player 1
        let row_participant_1 = match read_row(Player::<PARTICIPANT_1>) {
            Some(row) => row,
            None => {
                print!("done row_participant_1\n");
                break 'global;
            }
        };
        let mut id_participant_1 = *row_participant_1
            .get(0)
            .expect("cannot get id participant_1");

        // First fetch of id player 2
        let row_participant_2 = match read_row(Player::<PARTICIPANT_2>) {
            Some(row) => row,
            None => {
                print!("done row_participant_2\n");
                break 'global;
            }
        };
        let mut id_participant_2 = *row_participant_2
            .get(0)
            .expect("cannot get id participant_2");

        loop {
            println!("looop");
            if id_participant_1.eq(id_participant_2).reveal() {
                println!("match");
                // Send id to both PARTICIPANT_1 and PARTICIPANT_2
                SecretModp::from(id_participant_2)
                    .private_output(Player::<PARTICIPANT_1>, Channel::<0>);
                SecretModp::from(id_participant_2)
                    .private_output(Player::<PARTICIPANT_2>, Channel::<0>);

                println!("flush");
                // Send/Flush the row
                SecretModp::from(ConstI32::<0>)
                    .private_output(Player::<PARTICIPANT_1>, Channel::<1>);
                SecretModp::from(ConstI32::<0>)
                    .private_output(Player::<PARTICIPANT_2>, Channel::<1>);

                break; // It's good because we need to fetch new id_participant_1 and new id_participant_2
            } else if id_participant_1.lt(id_participant_2).reveal() {
                // Fetch next id_participant_1
                let row_participant_1 = match read_row(Player::<PARTICIPANT_1>) {
                    Some(row) => row,
                    None => {
                        print!("done row_participant_1\n");
                        break 'global;
                    }
                };
                id_participant_1 = *row_participant_1
                    .get(0)
                    .expect("cannot get id participant_1");
            } else if id_participant_2.lt(id_participant_1).reveal() {
                // Fetch next id_participant_2
                let row_participant_2 = match read_row(Player::<PARTICIPANT_2>) {
                    Some(row) => row,
                    None => {
                        print!("done row_participant_2\n");
                        break 'global;
                    }
                }; // [25, 0]
                id_participant_2 = *row_participant_2
                    .get(0)
                    .expect("cannot get id participant_2");
            }
        }
    }
}

fn read_row<const P: u32>(player: Player<P>) -> Option<Slice<SecretI64>> {
    // Because we have row like [[1], [2], [3]]
    let row_len = SecretModp::private_input(player, Channel::<0>).reveal();
    let row_len = i64::from(row_len) as u64;
    print!("row_len : ", row_len as i64, "\n");
    if row_len == 0 {
        return None;
    }

    let mut data = Slice::uninitialized(row_len);

    for row_nb in 0..row_len {
        let col_len = SecretModp::private_input(player, Channel::<1>).reveal();
        let col_len = i64::from(col_len) as u64;
        print!("col_len : ", col_len as i64, "\n");
        data.set(
            row_nb,
            &SecretModp::private_input(player, Channel::<2>).into(),
        );
    }

    Some(data)
}
