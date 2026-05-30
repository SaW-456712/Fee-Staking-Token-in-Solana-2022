# Fee-Staking-Token-in-Solana-2022
This code is designed to create a token based on Solana's Token-2022 standard.
This token incorporates features for transfer taxes and in-wallet staking.
The central figure is the Admin, who collects all tax proceeds into their own wallet and also possesses the authority to freeze and burn tokens.
Planned features include:
Airdrop management
Token control
Token metadata management

The code also handles the creation of wallets—both for the Admin and for the token itself—storing their keypairs in a JSON file for easy import into wallets like Phantom or Solflare.
Additionally, you can easily write a custom Oracle in Python to automatically adjust token parameters based on external data inputs.
It offers effortless deployment within a shell environment and simple management directly via the command line.
I believe this solution is ideal for creating your own lightweight, custom token.
I may later provide a sample Python Oracle implementation specifically for this token.
I might also release a version 2.0, which would allow you to fully customize and specify the token's parameters yourself.
The entire codebase is written in Rust and features Russian-language comments explaining every step of the process.

<img width="1224" height="970" alt="image" src="https://github.com/user-attachments/assets/99576104-b12c-4daa-bdc8-a6cc06b6fd9d" />

(я назвал этот проект как спираль так что где-то в коде может быть заметно )
