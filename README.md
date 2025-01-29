# Rustice

Rustice is a CLI tool for constrained LLM generation, built using [kalosm](https://github.com/floneum/floneum/tree/main/interfaces/kalosm). It is designed to process and parse structured text efficiently, leveraging CUDA for acceleration (Metal support is available but untested).

## 🚀 Features

- **Constrained LLM Generation**: Uses `kalosm` for structured text generation.
- **CLI Commands**:
  - `process` - Processes structured input data with a chosen LLM model.
  - `parse-html` - Parses raw HTML content and extracts relevant information.
- **CUDA Support**: Accelerated inference with CUDA (Metal should work, but untested on macOS).
- **Flexible Model Support**: Works with multiple models, my own test were made using `phi3-mini4k-instruct` as it fitted in VRAM.

---

## 📥 Setup Instructions

### Installation

Ensure you have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository:

```bash
git clone https://github.com/Shaamallow/Rustice.git
cd Rustice
```

### Download Required Data

```bash
wget https://drive.shaamallow.com/s/LEaBr6bq8qCDW3b
unzip data.zip -d data
```

### Build the Project

#### Default (no acceleration)

```bash
cargo build --release
```

#### With CUDA Acceleration

```bash
cargo build --release --features cuda
```

#### With Metal (macOS, untested)

```bash
cargo build --release --features metal
```

---

### Get my outputs (optional)

```bash
wget https://drive.shaamallow.com/s/D5D9i2ckC4GCPPS
unzip output.zip -d output
```

## 🛠 CLI Usage

### **Get Help**

Help is integrated into the CLI, you can access it by running:

```bash
./target/release/rustice --help
```

Or for specific commands:

```bash
./target/release/rustice <command> --help
```

### **Process Data**

```bash
./target/release/rustice process -i output/html_parsing -m phi3-mini4k-instruct -o output/llm_parsing
```

- `-i, --input` → Input folder
- `-m, --model` → Model to use (`phi3-mini4k-instruct`, `llama3-2_3b-chat`, etc.)
- `-o, --output` → Output folder

### **Parse HTML**

```bash
./target/release/rustice parse-html -i raw_html_folder -o output/html_parsing
```

- `-i, --input` → Folder containing raw HTML files
- `-o, --output` → Folder to store parsed output

---

## Potential Improvements

- **Testing & Metrics**: Develop proper evaluation metrics to determine the best performing model.
- **SFT LLM**: Implement Supervised Fine-Tuning (SFT) for improved constrained generation.
- **Deepseek Model Integration**: Extend support for the latest Deepseek models.
- **Data Curation & Task Understanding**: Improve dataset quality and formalize the problem domain.

_I'm already well above the allocated time for this task, but it was cool learning experience, especially about Rust and constrained LLM generation._ 🚀
