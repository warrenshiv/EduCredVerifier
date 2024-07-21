# EduCredVerifier

## Description

The Decentralized Credential Management System is a blockchain-based application designed to manage and verify academic credentials. Built using the Internet Computer platform, this system enables the creation, updating, revocation, and verification of academic credentials in a decentralized manner. It also manages institutions and students, ensuring secure and immutable records of academic achievements.

## Features

- **Create Credentials**: Issue new academic credentials linked to students and institutions.
- **Revoke Credentials**: Mark credentials as revoked to prevent their further use.
- **Update Credentials**: Modify existing credentials with updated information.
- **Verify Credentials**: Confirm the validity of a credential based on student and institution details.
- **Manage Institutions**: Create and manage institutions that issue credentials.
- **Manage Students**: Add and manage student records with unique email verification.
- **Search and Retrieve Records**: Efficiently search for and retrieve credentials, students, and institutions by various criteria.

## Hardcoded Token

To authenticate API requests, the system uses a hardcoded token:

- **Token**: `supersecrettoken`

Ensure that the token provided in requests matches the predefined token to gain access.

## How to Use

1. **Create Credentials**: Use the `create_credential` function with the required details to issue new credentials.
2. **Revoke Credentials**: Call `revoke_credential` to invalidate a credential that should no longer be valid.
3. **Update Credentials**: Modify existing credentials using `update_credential` to reflect any changes.
4. **Verify Credentials**: Check the validity of a credential using the `verify_credential` function.
5. **Manage Institutions and Students**: Add new institutions and students using `create_institution` and `create_student`, respectively.

## API Endpoints

- `create_credential(payload: CredentialPayload)`: Issues a new credential.
- `revoke_credential(payload: RevokeCredentialPayload)`: Revokes an existing credential.
- `update_credential(payload: UpdateCredentialPayload, token: String)`: Updates an existing credential.
- `get_credentials()`: Retrieves all credentials.
- `get_credential_by_id(id: u64)`: Fetches a credential by its ID.
- `search_credentials(payload: SearchCredentialPayload)`: Searches for credentials based on given criteria.
- `create_institution(payload: InstitutionPayload)`: Adds a new institution.
- `get_institutions()`: Retrieves all institutions.
- `get_institution_by_id(id: u64)`: Fetches an institution by its ID.
- `create_student(payload: StudentPayload)`: Adds a new student.
- `get_students()`: Retrieves all students.
- `get_student_by_id(id: u64)`: Fetches a student by their ID.
- `verify_credential(payload: VerifyPayload)`: Verifies the validity of a credential.




## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```