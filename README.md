Here is a clean, scannable README.md refactor designed so anyone landing on the repo immediately grasps what **osracg** does, how to run it, and how versions evolved—without getting bogged down in dense walls of text. (Also caught and fixed a small typo in your original Windows --update snippet where it had .\osrscg --update.)
```markdown
# osracg

**OS Recorder Action Code Generator** — Record keyboard and mouse actions on your OS and automatically generate a standalone **Python** (`pyautogui`) or **Rust** (`enigo`) script.

Run the generated file directly or import/include it as a module in your own project.

> **Demo (v0.1.9):** [Watch on YouTube](https://youtu.be/BXep_h8a7nY?si=BWVwMDX9v9g2CKTW)

---

## Quick Start

### Prerequisites
* **Python output:** `pip install pyautogui`
* **Rust output:** Add `enigo` to your project dependencies

### 1. Download & Open Terminal
Download the release binary for your OS and open your terminal or Command Prompt in that same directory.

### 2. Start Recording
Flags after `<your_file_name>` are optional.

**Linux**
```bash
./osracg <your_file_name> [-py | -rs | --path <target_dir>]

```
**Windows**
```powershell
.\osracg <your_file_name> [-py | -rs | --path <target_dir>]

```
 * Default output format is -py (Python).
 * Use -rs to generate a Rust file.
 * Use --path <target_dir> to output the generated file to a specific folder.
### 3. Record & Stop
 1. **Put your target window in the background first** — osracg automatically triggers Alt+Tab when starting (this Alt+Tab is not recorded).
 2. Perform your keyboard and mouse actions.
 3. Press **F12** to stop recording and generate <your_file_name>.py or <your_file_name>.rs.
### 4. Use the Generated File
Run the generated script standalone, or place it alongside your project and call osracg():
**Python**
```python
import your_file_name

your_file_name.osracg()

```
**Rust**
```rust
mod your_file_name;

fn main() {
    your_file_name::osracg();
}

```
## Other Commands
**Linux**
```bash
./osracg -v          # Show version
./osracg -h          # Show help
./osracg --upgrade   # Self-update binary
./osracg --update    # Same as --upgrade

```
**Windows**
```powershell
.\osracg -v          # Show version
.\osracg -h          # Show help
.\osracg --upgrade   # Self-update binary
.\osracg --update    # Same as --upgrade

```
## Notes & Limitations
 * **Supported keys:** See Support_Keys.txt.
 * **Mouse wheel:** Not supported.
 * **Key holds:** Avoid holding keys down (such as Delete) for long periods, as replay timing can have minor inaccuracies.
 * **Intentional pauses:** Pause for **at least 2 seconds** during recording if you want a delay written into the generated script.
 * **Changelog details:** See Version Note.txt.
## Release History
| Version | Highlights |
|---|---|
| **v0.4.1** | Patched a bug causing --update / --upgrade to fail. |
| **v0.4.0** | Added Rust (enigo) code generation and --path output directory support. |
| **v0.3.1** | Added --update alias matching --upgrade. |
| **v0.3.0** | Added self-upgrade support via --upgrade. |
| **v0.2.1** | Improved osracg_core for smoother recording and lower lag. |
| **v0.2.0** | Fixed Windows compatibility, embedded osracg_core into a single binary, and added smarter pause detection (pause ≥ 2s to record a delay). |
| **v0.1.9** | Fixed backslash (\) handling and slightly tuned input durations for reliability. |
| **v0.1.8** | Faster Python execution and updated usage workflow. |
| **v0.1.7** | Trimmed leading mouse movements prior to clicks and added AltGr support. |
| **v0.1.6** | Fixed Ctrl key handling issues. |
| **v0.1.5** | Patched a major crash bug (same fix line as v0.1.4). |
| **v0.1.4** | Patched a major crash bug. |
| **v0.1.3** | Fixed -v unintentionally starting a recording session. |
| **v0.1.2** | Added -v (version) and -h (help) flags. |
| **v0.1.1** | Smoothed typing replay. |
| **v0.1.0** | Initial release with Python code generation and custom module naming. |
