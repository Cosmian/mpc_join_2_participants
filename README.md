# MPC Inner Join with 2 participants

This code can be used in [Cosmian CipherCompute](https://github.com/Cosmian/CipherCompute) app to run an inner join over an MPC system.

In this example, 2 participants are providing data and receiving the results of the inner join operation.

The 3rd participant is only here as an arbiter, as the MPC system needs at least 3 participants to run a computation.

The input data is expected to be sorted before running the computation.

## Example

Input data:

| Participant 0 | Participant 1 | Participant 2 |
| :-----------: | :-----------: | :-----------: |
|       1       |       3       |               |
|       2       |       4       |               |
|       3       |       5       |               |
|       4       |       7       |               |
|       5       |      11       |               |
|       6       |      13       |               |

Output results of the inner join operation:

| Participant 0 | Participant 1 |
| :-----------: | :-----------: |
|       3       |       3       |
|       4       |       4       |
|       5       |       5       |


## Hack it !

The code is heavily documented and under MIT license as it is meant to be hacked for your purpose.

It is actually very easy to generalize this code to a lot of confidential datasets generation problems.

Do not hesitate to open issues and PRs to improve the quality of this code 
and its documentation.

## Editing and testing

Once you have cloned this repository locally, edit the code; 
we recommend that you use the free VSCode and rust-analyzer extension.

To check the validity of your code, simply run  `cargo build`. 
The build process outputs [WASM](https://fr.wikipedia.org/wiki/WebAssembly) which
is what is actually fed as an intermediate representation to the CipherCompute engine.

To facilitate testing without having to run [CipherCompute EAP](https://github.com/Cosmian/CipherCompute),  2 facilities are provided via 2 scripts:

 - `emulate.sh` will pick up the input data in the `data/inputs` directory 
  and output its results in the `data/outputs` directory. These directories contain one 
  file per player. This scripts perform the same emulation as that provided on the CipherCompute UI. 

 - `test.sh` will run the unit tests of the `main.rs` file. For a test written 
   ```rust
   #[test]
    fn test_example() {
        // An example of a successful test
        // which input and expected output data are located
        // in the `fixtures/first_test` folder
        cosmian_std::test!("first_test");
        // If you change any data in the input or output files,
        // the test will fail
    }
    ```
    The input data will be picked up from the `fixtures/first_test/inputs` directory and
    the outputs will be **compared** to those of the `fixtures/first_test/outputs` directory.

## Testing inside the CipherCompute MPC engine

1. Make a change and test it using `./simulate.sh`
2. commit the change to the local git and note the git commit

3. Then use the `git-daemon.sh` script to launch a git daemon which exposes this project at
at a git URL displayed by the script

From the UI on the CipherCompute EAP version

4. Create/update a computation using the git URL above and the git commit you want to test
5. Run the computation from the UI

See the [CipherCompute EAP](https://github.com/Cosmian/CipherCompute) Quick Start Guide
on how to use its UI to configure a computation.
