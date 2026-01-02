# Architecture Documentation

This document describes the high-level architecture and logic flow of `chrome-pwd-dumper-rs`, a Rust-based tool for dumping saved passwords from Chromium-based browsers on Windows.

## Overview

The application is a command-line interface (CLI) tool that interacts with browser data files (specifically `Local State` and `Login Data`) to extract and decrypt user credentials. It relies on Windows APIs (DPAPI) for key decryption and standard crypto libraries for AES-GCM decryption.

## Core Modules

*   **`main.rs`**: Entry point. Handles argument parsing, orchestration of the dumping process, and output generation.
*   **`dumper.rs`**: implementation of the `Dumper` struct. Responsible for locating browser files, querying the SQLite database, and managing the decryption workflow for a specific browser.
*   **`decryption_core.rs`**: Low-level cryptographic primitives. Wraps Windows DPAPI (`CryptUnprotectData`) and implements AES-256-GCM decryption.
*   **`models.rs`**: Data structures defining the schema for raw browser data (`ChromeAccount`, `LocalState`) and the final output (`DecryptedAccount`).
*   **`args.rs`**: Command-line argument definition using `argh`.

## Application Flow

### 1. High-Level Execution Flow

The following diagram illustrates the lifecycle of the application from execution to termination.

```mermaid
graph TD
    A[Start: main.rs] --> B{Parse Arguments}
    B --> C[Create Temp Dir ./.tmp]
    C --> D[Initialize supported Browsers Map]
    D --> E{User selected specific browsers?}
    E -- Yes --> F[Filter Browsers List]
    E -- No --> G[Use All Supported Browsers]
    F --> H
    G --> H[Iterate over Browsers]
    
    subgraph Processing Loop
        H --> I[Get Dumper Instance]
        I --> J[Call Dumper.dump]
        J --> K{Success?}
        K -- Yes --> L[Collect Decrypted Accounts]
        K -- No --> M[Skip / Log Error]
    end
    
    L --> N{Output Format?}
    N -- JSON --> O[Serialize to JSON]
    N -- Text --> P[Format as Text]
    O --> Q[Write to File]
    P --> Q
    Q --> R{Print to Stdout?}
    R -- Yes --> S[Print Data]
    R -- No --> T[Cleanup Temp Dir]
    S --> T
    T --> U[End]
```

### 2. Dumping and Decryption Logic

This diagram details the internal logic within `Dumper.dump()`, showing how keys are retrieved and how passwords are decrypted depending on the Chrome version mechanism (pre or post v80).

```mermaid
graph TD
    Start[Dumper.dump] --> LocateLS[Locate 'Local State' file]
    LocateLS --> ReadLS{File Exists?}
    
    %% Chrome v80+ Path (Master Key)
    ReadLS -- Yes --> ExtractKey[Read 'os_crypt.encrypted_key']
    ExtractKey --> DecodeKey[Base64 Decode Key]
    DecodeKey --> DecryptKey[Decrypt Key using Windows DPAPI]
    DecryptKey --> GotMasterKey[Obtain Master Key]
    
    %% Common Path: Database Reading
    ReadLS -- No --> CopyDB
    GotMasterKey --> CopyDB[Copy 'Login Data' SQLite DB to .tmp]
    CopyDB --> ConnectDB[Open SQLite Connection]
    ConnectDB --> QueryDB[SELECT action_url, username, password FROM logins]
    QueryDB --> IterateRows[Iterate Account Rows]
    
    subgraph Decryption Strategy
        IterateRows --> CheckMethod{Has Master Key?}
        
        %% AES-GCM Path
        CheckMethod -- Yes --> DecryptAES[Decrypt Password using AES-256-GCM]
        DecryptAES --> Validate[Check Output]
        
        %% Fallback / Legacy DPAPI Path
        CheckMethod -- No --> DecryptDPAPI[Decrypt Password using Windows DPAPI]
        DecryptAES -- Error --> DecryptDPAPI
        
        DecryptDPAPI --> Result{Success?}
        Result -- Yes --> ReturnAcc[Return DecryptedAccount]
        Result -- No --> Skip[Skip Account]
    end
    
    ReturnAcc --> Collect[Add to Results List]
    Skip --> IterateRows
```

## Key Components Detail

### Browser Detection
The tool uses a predefined list of `AppInfo` structures mapping known browser names (e.g., "Chrome", "Edge", "Brave") to their expected directory paths within the user's `AppData`.

### Cryptography
1.  **Master Key (Chrome v80+)**:
    *   Chrome stores a master key in the `Local State` JSON file.
    *   This key starts with `DPAPI` and is encrypted using Windows DPAPI.
    *   The tool decrypts this key using `CryptUnprotectData`.
    *   The resulting *Master Key* is used to decrypt the actual passwords.

2.  **Password Decryption**:
    *   **Modern (v80+)**: Passwords are encrypted with AES-256-GCM. The initialization vector (IV) is extracted from the password field (bytes 3-15), and the payload follows. The *Master Key* is used for this decryption.
    *   **Legacy**: Passwords were directly encrypted using DPAPI. The tool attempts this if no Master Key is found or as a fallback.
