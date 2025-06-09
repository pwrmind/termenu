# Rust File Manager

A simple command-line file manager written in Rust. This application allows users to navigate through directories, create files and folders, and manage files using clipboard operations (copy, cut, paste).

## Features

- Navigate through directories
- Create new files and folders
- Copy and cut files to a clipboard
- Paste files from the clipboard
- Delete and rename files
- Open files with the default application

## Requirements

- Rust (1.50 or later)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/pwrmind/termenu.git
   cd termenu
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the application:

   ```bash
   ./target/release/termenu
   ```

## Usage

Upon running the application, you will see a menu displaying the contents of the current directory. You can perform the following actions:

```bash
Текущий путь: projects/rust/menu

📋 Буфер: Копирование 'menu.pdb'

[0] 📁 ..
[1] 📁 .git
[2] 📁 src
[3] 📁 target
[4] 📄 .gitignore
[5] 📄 Cargo.lock
[6] 📄 Cargo.toml
[7] 📄 menu.d
[8] 📄 menu.pdb

Системные команды:
[p] Вставить из буфера
[c] Очистить буфер
[n] Создать папку
[f] Создать файл
[q] Выход

Выберите пункт или команду: q
```

- Select a directory or file by entering the corresponding index.
- Use the following commands:
  - `[p]` Paste from clipboard (if not empty)
  - `[c]` Clear clipboard
  - `[n]` Create a new folder
  - `[f]` Create a new file
  - `[q]` Exit the application

When a file is selected, you can perform additional actions:

- `[1]` Copy the file to clipboard
- `[2]` Cut the file to clipboard
- `[3]` Delete the file
- `[4]` Rename the file
- `[5]` Open the file with the default application

## Example

1. Navigate to a directory.
2. Create a new folder named "NewFolder".
3. Create a new file named "example.txt".
4. Copy "example.txt" to the clipboard.
5. Paste it in the current directory.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Acknowledgments

- Rust programming language
- The community for their support and resources
