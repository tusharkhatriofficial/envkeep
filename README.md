<p align="center">
  <h1 align="center">envkeep</h1>
  <p align="center">
    <strong>Your .env files, encrypted locally. No accounts. No cloud. Just works.</strong>
  </p>
  <p align="center">
    <a href="#status">Status</a> &bull;
    <a href="#quick-start">Quick Start</a> &bull;
    <a href="#commands">Commands</a> &bull;
    <a href="#why">Why</a> &bull;
    <a href="#security">Security</a>
  </p>
</p>

<br>

You have 12 projects on your machine. Each has a `.env` file. Half the secrets are copy-pasted between them. You cannot search across them. You cannot diff them. You cannot back them up. One bad `rm -rf` and they are gone.

**envkeep fixes this in 30 seconds:**

```bash
envkeep init                                 # set a master password
cd ~/code/my-saas && envkeep add my-saas     # encrypt and store .env
cd ~/code/api && envkeep add api             # again
cd ~/code/landing && envkeep add landing     # and again

# later, new laptop, whatever:
envkeep use my-saas                          # .env restored
```

Every value is encrypted with AES-256-GCM, stored in an encrypted SQLCipher database, and unlocked with a single master password.

---

## Quick Install 
### macOS/Linux

`curl -fsSL https://raw.githubusercontent.com/tusharkhatriofficial/envkeep/main/install.sh | bash`

### Windows PowerShell
  
`irm https://raw.githubusercontent.com/tusharkhatriofficial/envkeep/main/install.ps1 | iex`

## What it looks like

```
$ envkeep list

  envkeep projects (4)

┌─────────────────┬──────┬──────────┐
│ Project         │ Vars │ Modified │
├─────────────────┼──────┼──────────┤
│ my-saas         │ 28   │ 2h ago   │
│ api             │ 45   │ 3h ago   │
│ landing-page    │ 12   │ 1d ago   │
│ worker          │ 8    │ 2d ago   │
└─────────────────┴──────┴──────────┘

$ envkeep search DATABASE_URL

  Found DATABASE_URL in 3 projects:
  |-- my-saas: postgresql://localhost/saas_dev
  |-- api: postgresql://localhost/api_dev
  |-- worker: postgresql://localhost/worker_dev

$ envkeep inspect my-saas

  Project: my-saas (28 variables)

┌──────────────────┬──────────────────────────────┐
│ Key              │ Value                        │
├──────────────────┼──────────────────────────────┤
│ DATABASE_URL     │ postgresql://localhost/*****  │
│ REDIS_URL        │ redis://localhost:6379/*****  │
│ STRIPE_SECRET_KEY│ ********                     │
│ APP_PORT         │ 3000                         │
│ DEBUG            │ true                         │
└──────────────────┴──────────────────────────────┘

$ envkeep use my-saas
  Wrote 28 variables to .env
```

---

## Status

**envkeep is under active development (v0.2.0-alpha released).**

To try it now, build from source:

```bash
git clone https://github.com/tusharkhatriofficial/envkeep.git
cd envkeep
cargo build --release
./target/release/envkeep --help
```

Requires Rust 1.70+ and a C compiler (for SQLCipher). Pre-built binaries and `cargo install` will be available at first stable release.

---

## Quick start

**1. Create your vault**
```
$ envkeep init
  Creating a new vault. Choose a master password.
  Enter master password: --------
  Confirm master password: --------
  Vault created at ~/.envkeep/vault.db
```

**2. Store a project**
```
$ cd ~/code/my-saas
$ envkeep add my-saas
  Added project my-saas with 28 variables
```

**3. Restore it anywhere**
```
$ cd ~/code/my-saas
$ envkeep use my-saas
  Wrote 28 variables to .env
```

---

## Commands

### Core

| Command | Description |
|---|---|
| `envkeep init` | Create encrypted vault, set master password |
| `envkeep add <name>` | Read `.env` from current directory, encrypt, store |
| `envkeep use <name>` | Write decrypted `.env` to current directory |
| `envkeep list` | List all projects |
| `envkeep remove <name>` | Delete a project from the vault |

### Inspect and compare

| Command | Description |
|---|---|
| `envkeep inspect <name>` | Show variables with secrets masked |
| `envkeep diff <a> <b>` | Compare variables between two projects |
| `envkeep search <key>` | Find which projects use a given key |
| `envkeep unused <name>` | Find variables not referenced in source code |
| `envkeep validate <name>` | Check for common mistakes (bad ports, malformed URLs) |
| `envkeep types <name>` | Infer variable types (string, number, boolean, URL) |

### Secrets and sharing

| Command | Description |
|---|---|
| `envkeep secrets set KEY=VALUE` | Store an encrypted shared secret |
| `envkeep secrets list` | List all secrets (values masked) |
| `envkeep secrets link <key> <project>` | Link a shared secret to a project |
| `envkeep sync <from> <to>` | Copy common variables between projects |
| `envkeep export <name>` | Export project as encrypted `.envvault` file |
| `envkeep import <file>` | Import from `.envvault` file |

### Backup and ops

| Command | Description |
|---|---|
| `envkeep backup` | Full vault backup (encrypted) |
| `envkeep restore <file>` | Restore vault from backup |
| `envkeep status` | Show active project |
| `envkeep recent` | Switch to recently used project |

### Terminal UI

```
$ envkeep tui
```

Full-screen terminal interface. Navigate projects, edit variables, search across the vault.

```
+-  envkeep  -----------------------------------+
|  Projects (4)                                  |
|                                                |
|  > my-saas       28 vars    2h ago             |
|    api           45 vars    3h ago             |
|    landing-page  12 vars    1d ago             |
|    worker         8 vars    2d ago             |
|  ------------------------------------------   |
|  /  Search    e  Edit    s  Sync    q  Quit    |
+------------------------------------------------+
```

---

## Why

Every developer has this problem:

```
~/code/
  my-saas/.env          # 28 vars, half copy-pasted from api/
  api/.env              # 45 vars, STRIPE_KEY duplicated in 3 places
  landing/.env          # forgot to update DATABASE_URL after migration
  worker/.env           # is this the right REDIS_URL?
  side-project/.env     # what is even in here?
```

No search. No diff. No backup. Just scattered plaintext files with production credentials in them.

envkeep replaces all of that with one encrypted file:

```
~/.envkeep/vault.db
```

### Comparison

| | envkeep | Doppler | Infisical | direnv |
|---|---|---|---|---|
| Cost | Free | $20+/mo | $10+/mo | Free |
| Storage | Local | Cloud | Cloud | .envrc files |
| Encryption | AES-256-GCM + SQLCipher | Server-side | Server-side | None |
| Account required | No | Yes | Yes | No |
| Cross-project search | Yes | Limited | Limited | No |
| Dead variable detection | Yes | No | No | No |
| Terminal UI | Yes | No | No | No |

---

## Security

```
Master Password
      |
      v  PBKDF2-HMAC-SHA256 (100,000 iterations) + random salt
      |
 Derived Key (32 bytes)
      |
      +---> SQLCipher (encrypts entire database file)
      |
      +---> AES-256-GCM (encrypts each variable value individually)
```

- **Master password is never stored.** Only a verification hash derived via PBKDF2.
- **Double encryption.** The database file is encrypted with SQLCipher. Each value inside is encrypted separately with AES-256-GCM and a unique nonce.
- **Zero plaintext on disk.** Nothing in the vault is ever stored unencrypted.
- **Tamper detection.** GCM mode provides authenticated encryption. Any modification to ciphertext is detected and rejected.
- **Cryptography by [`ring`](https://github.com/briansmith/ring)** -- the same library behind rustls, Cloudflare, and Fastly.

---

## How it works

```
envkeep add my-saas:
  1. Read .env from current directory
  2. Parse KEY=VALUE pairs (handles quotes, comments, inline comments)
  3. Encrypt each value individually with AES-256-GCM
  4. Store in ~/.envkeep/vault.db (SQLCipher-encrypted database)

envkeep use my-saas:
  1. Unlock vault with master password
  2. Decrypt each variable
  3. Write .env to current directory
```

---

## Built with

| | |
|---|---|
| [clap](https://github.com/clap-rs/clap) | CLI argument parsing |
| [ring](https://github.com/briansmith/ring) | AES-256-GCM encryption, PBKDF2 key derivation |
| [rusqlite](https://github.com/rusqlite/rusqlite) + SQLCipher | Encrypted database |
| [ratatui](https://github.com/ratatui/ratatui) | Terminal UI |
| [rpassword](https://github.com/conradkleinespel/rpassword) | Hidden password input |

---

## Contributing

```bash
git clone https://github.com/tusharkhatriofficial/envkeep.git
cd envkeep
cargo build
cargo test
cargo run -- init
```

---

## License

MIT
