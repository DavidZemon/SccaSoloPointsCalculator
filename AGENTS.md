# SCCA Solo Points Calculator – Agent Guide

## Project Overview

This is a browser-based tool that calculates SCCA (Sports Car Club of America) Solo autocross championship points. Users
upload CSV/XLS files exported from MotorsportReg (MSR) and Pronto (TSAdmin/TSAnnounce) to generate per-event class
results and running championship standings.

The application runs entirely in the browser — there is no server. Heavy business logic is compiled to **WebAssembly**
from Rust and loaded by the React frontend.

---

## Tech Stack

| Layer             | Technology                                                            |
|-------------------|-----------------------------------------------------------------------|
| Frontend UI       | React 17, TypeScript 4, React-Bootstrap (Bootswatch Slate theme)      |
| Business Logic    | Rust 1.88 compiled to WebAssembly via `wasm-bindgen`                  |
| Build tool (JS)   | Create React App (`react-scripts`)                                    |
| Build tool (Rust) | `cargo` + `wasm-bindgen` CLI                                          |
| Package manager   | Yarn (v1)                                                             |
| Formatting        | `prettier` (JS/TS), `rustfmt` (Rust)                                  |
| Testing           | `react-scripts test` (JS), `cargo test` (Rust / `rstest` + `mockall`) |

---

## Repository Layout

```
SccaPointsCalculator/
├── src/                        # React/TypeScript frontend
│   ├── App.tsx                 # Root component; orchestrates file uploads & state
│   └── components/
│       ├── FileUploadBox.tsx   # Reusable drag-and-drop / click-to-upload widget
│       ├── EventResults.tsx    # Renders per-class event results tabs
│       ├── ChampionshipResults.tsx  # Renders championship standings tabs
│       ├── CsvTable.tsx        # Generic CSV → HTML table renderer
│       └── DownloadButton.tsx  # CSV download helper
├── src-rs/                     # Rust source (compiled to WASM)
│   ├── lib.rs                  # Public WASM API – `SccaSoloPointsEngine`
│   ├── enums/                  # ChampionshipType, DriverGroup, car-class enums
│   ├── models/                 # Domain models (Driver, LapTime, EventResults, etc.)
│   ├── services/
│   │   ├── csv/
│   │   │   ├── parser/         # Parses MSR exports, Pronto exports, championship XLS files
│   │   │   └── builder/        # Builds output CSV strings from EventResults
│   │   └── calculators/        # Points, trophy, and tie-breaking calculations
│   └── utilities/
├── rusty/                      # wasm-bindgen output (generated – do NOT edit manually)
│   ├── scca_solo_points_engine.js
│   ├── scca_solo_points_engine.d.ts
│   └── scca_solo_points_engine_bg.wasm
├── public/                     # CRA public assets; also holds the .wasm file at runtime
├── build/                      # Production build output (generated)
├── SampleData/                 # Real-world CSV/XLS test fixtures, organized by year
├── Cargo.toml                  # Rust crate manifest
├── rust-toolchain.toml         # Pins Rust channel (1.88) and wasm32 target
└── package.json                # JS dependencies and npm scripts
```

---

## Architecture

### Data flow

```
User uploads files
      │
      ▼
App.tsx (React state)
      │  MSR CSV + Pronto CSV (as strings)
      ▼
SccaSoloPointsEngine::new()  ← Rust/WASM constructor
      │  returns engine instance
      ▼
  ┌─────────────────────────────────────┐
  │  engine.js_drivers_in_error()       │  Error check before displaying results
  │  engine.get_event_class_results_csvs()   │  Per-class CSV strings
  │  engine.get_event_combined_csv()    │  PAX / combined CSV string
  │  engine.add_prior_championship_results() │  Merges prior XLS standings
  └─────────────────────────────────────┘
      │  CSV strings
      ▼
CsvTable component renders HTML tables
DownloadButton offers file download
```

### Rust/WASM public API (`src-rs/lib.rs`)

The sole public struct is `SccaSoloPointsEngine`. All methods exposed to JavaScript are annotated `#[wasm_bindgen]`.

| Method                                              | Purpose                                                                       |
|-----------------------------------------------------|-------------------------------------------------------------------------------|
| `new(msr_export, pronto_export)`                    | Parse both CSVs, build `EventResults`; returns `Err(String)` on parse failure |
| `js_drivers_in_error()`                             | Returns drivers whose class info couldn't be resolved                         |
| `get_header_for_event_class_results()`              | CSV header string for class results                                           |
| `get_event_class_results_csvs()`                    | One CSV string per car class                                                  |
| `get_event_combined_csv(driver_group)`              | Combined/PAX results for a `DriverGroup`                                      |
| `add_prior_championship_results(type, bytes, name)` | Merge XLS championship standings; returns updated CSV                         |

### Key Rust modules

- **`enums/championship_type.rs`** – `Class | PAX | Novice | Ladies`
- **`enums/driver_group.rs`** – groupings used for combined results
- **`models/event_results.rs`** – central aggregate of all parsed drivers/classes
- **`services/csv/parser/event_results_parser.rs`** – entry point for parsing MSR + Pronto exports
- **`services/csv/parser/championship_results_parser.rs`** – merges XLS prior standings with current event
- **`services/calculators/championship_points_calculator.rs`** – SCCA points formula
- **`services/calculators/trophy_calculator.rs`** – trophy count per class
- **`services/calculators/tie_calculator.rs`** – tie-breaking logic

---

## Build & Development Commands

### Full dev cycle (builds Rust → WASM, then starts CRA dev server)

```bash
yarn start
```

### Build Rust/WASM only

```bash
yarn build-rs
# Equivalent to:
cargo build --release --target wasm32-unknown-unknown \
  && wasm-bindgen target/wasm32-unknown-unknown/release/scca_solo_points_engine.wasm \
     --target=web --out-dir=rusty --omit-default-module-path \
  && cp rusty/scca_solo_points_engine_bg.wasm public/
```

### Production build

```bash
yarn build
# Sets PUBLIC_URL=https://david.zemon.name/pronto before react-scripts build
```

### Run JS tests

```bash
yarn test
```

### Run Rust unit tests (native, not WASM)

```bash
cargo test
```

### Format code

```bash
# Rust
cargo fmt

# TypeScript / TSX
yarn prettier --write "src/**/*.{ts,tsx}"
```

---

## Important Conventions & Gotchas

1. **`rusty/` is generated.** Never edit files in `rusty/` directly. They are overwritten every time `yarn build-rs`
   runs.
2. **The `.wasm` file must be in `public/`** at dev time and in `build/` for production — the build scripts handle this
   automatically via `cp`.
3. **Rust crate type is `["cdylib", "rlib"]`** — `cdylib` for the WASM binary, `rlib` so that `cargo test` works for
   unit tests without a browser.
4. **No backend.** All parsing, calculation, and rendering happens client-side. Do not add server-side infrastructure
   without discussing the approach first.
5. **Championship standings are XLS files** (`.xls`/`.xlsx`) read via the `calamine` Rust crate on the WASM side.
6. **Error handling pattern:** Rust functions return `Result<T, String>` or `Result<T, JsValue>`; the React layer
   catches these and displays toast notifications via `react-toastify`.
7. **Rust toolchain is pinned** to channel `1.88` with the `wasm32-unknown-unknown` target in `rust-toolchain.toml`. Do
   not change this without verifying `wasm-bindgen` compatibility.
8. **Sample data** is in `SampleData/` (and year subdirectories). Use these files when writing or updating tests.
9. **Yarn v1** is used — use `yarn add` (not `npm install`) to add JS dependencies.

