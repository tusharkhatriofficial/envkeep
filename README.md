<p align="center">
  <h1 align="center">dotkeep</h1>
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

**dotkeep fixes this in 30 seconds:**

```bash
dotkeep init                                 # set a master password
cd ~/code/my-saas && dotkeep add my-saas     # encrypt and store .env
cd ~/code/api && dotkeep add api             # again
cd ~/code/landing && dotkeep add landing     # and again

# later, new laptop, whatever:
dotkeep use my-saas                          # .env restored
```

Every value is encrypted with AES-256-GCM, stored in an encrypted SQLCipher database, and unlocked with a single master password.

---

## What it looks like

```
$ dotkeep list

  dotkeep projects (4)

┌─────────────────┬──────┬──────────┐
│ Project         │ Vars │ Modified │
├─────────────────┼──────┼──────────┤
│ my-saas         │ 28   │ 2h ago   │
│ api             │ 45   │ 3h ago   │
│ landing-page    │ 12   │ 1d ago   │
│ worker          │ 8    │ 2d ago   │
└─────────────────┴──────┴──────────┘

$ dotkeep search DATABASE_URL

  Found DATABASE_URL in 3 projects:
  |-- my-saas: postgresql://localhost/saas_dev
  |-- api: postgresql://localhost/api_dev
  |-- worker: postgresql://localhost/worker_dev

$ dotkeep inspect my-saas

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

$ dotkeep use my-saas
  Wrote 28 variables to .env
```

---

## Status

**dotkeep is under active development and has not been released yet.**

To try it now, build from source:

```bash
git clone https://github.com/tusharkhatriofficial/dotkeep.git
cd dotkeep
cargo build --release
./target/release/dotkeep --help
```

Requires Rust 1.70+ and a C compiler (for SQLCipher). Pre-built binaries and `cargo install` will be available at first stable release.

---

## Quick start

**1. Create your vault**
```
$ dotkeep init
  Creating a new vault. Choose a master password.
  Enter master password: --------
  Confirm master password: --------
  Vault created at ~/.dotkeep/vault.db
```

**2. Store a project**
```
$ cd ~/code/my-saas
$ dotkeep add my-saas
  Added project my-saas with 28 variables
```

**3. Restore it anywhere**
```
$ cd ~/code/my-saas
$ dotkeep use my-saas
  Wrote 28 variables to .env
```

---

## Commands

### Core

| Command | Description |
|---|---|
| `dotkeep init` | Create encrypted vault, set master password |
| `dotkeep add <name>` | Read `.env` from current directory, encrypt, store |
| `dotkeep use <name>` | Write decrypted `.env` to current directory |
| `dotkeep list` | List all projects |
| `dotkeep remove <name>` | Delete a project from the vault |

### Inspect and compare

| Command | Description |
|---|---|
| `dotkeep inspect <name>` | Show variables with secrets masked |
| `dotkeep diff <a> <b>` | Compare variables between two projects |
| `dotkeep search <key>` | Find which projects use a given key |
| `dotkeep unused <name>` | Find variables not referenced in source code |
| `dotkeep validate <name>` | Check for common mistakes (bad ports, malformed URLs) |
| `dotkeep types <name>` | Infer variable types (string, number, boolean, URL) |

### Secrets and sharing

| Command | Description |
|---|---|
| `dotkeep secrets set KEY=VALUE` | Store an encrypted shared secret |
| `dotkeep secrets list` | List all secrets (values masked) |
| `dotkeep secrets link <key> <project>` | Link a shared secret to a project |
| `dotkeep sync <from> <to>` | Copy common variables between projects |
| `dotkeep export <name>` | Export project as encrypted `.envvault` file |
| `dotkeep import <file>` | Import from `.envvault` file |

### Backup and ops

| Command | Description |
|---|---|
| `dotkeep backup` | Full vault backup (encrypted) |
| `dotkeep restore <file>` | Restore vault from backup |
| `dotkeep status` | Show active project |
| `dotkeep recent` | Switch to recently used project |

### Terminal UI

```
$ dotkeep tui
```

Full-screen terminal interface. Navigate projects, edit variables, search across the vault.

```
+-  dotkeep  -----------------------------------+
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

dotkeep replaces all of that with one encrypted file:

```
~/.dotkeep/vault.db
```

### Comparison

| | dotkeep | Doppler | Infisical | direnv |
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
dotkeep add my-saas:
  1. Read .env from current directory
  2. Parse KEY=VALUE pairs (handles quotes, comments, inline comments)
  3. Encrypt each value individually with AES-256-GCM
  4. Store in ~/.dotkeep/vault.db (SQLCipher-encrypted database)

dotkeep use my-saas:
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
git clone https://github.com/tusharkhatriofficial/dotkeep.git
cd dotkeep
cargo build
cargo test
cargo run -- init
```

---

## License

MIT
