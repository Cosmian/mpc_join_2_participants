# MPC Join with 2 participants

This code can be used in [Cosmian CipherCompute](https://github.com/Cosmian/CipherCompute) app to run an inner join over an MPC system.

In this implementation, 2 participants are providing data and receiving the results of the join operation.

The 3rd participant is only here as an arbiter, as the MPC system needs at least 3 participants to run a computation.

We consider input data to be sorted before running the computation.

## Example

Input data:

| Participant 1 | Participant 2 | Participant 3 |
| :-----------: | :-----------: | :-----------: |
|       1       |       3       |               |
|       2       |       4       |               |
|       3       |       5       |               |
|       4       |       7       |               |
|       5       |      11       |               |
|       6       |      13       |               |

Output results of the inner join operation:

| Participant 1 | Participant 2 |
| :-----------: | :-----------: |
|       3       |       3       |
|       4       |       4       |
|       5       |       5       |
