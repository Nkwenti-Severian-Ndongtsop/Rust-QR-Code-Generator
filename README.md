# Rust-QR-Code-Generator

A blazing-fast and simple command-line application to generate qr-code based on user arguments using Rust.

## ✨ Features
- 🚀 Many image formats
- 🔍 Supports command-line input for data
- 🌍 Free size argument input

## 🛠️ Installation
### Prerequisites
- 🦀 Rust (latest stable version recommended)

### ▶️ Run the CLI

-  add this **export PATH="$HOME/.cargo/bin:$PATH"** to you shell configuration file e.g: ~/.zshrc ~/.bashrc

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

```sh
qr-code [data] [format] [size]
```

Example:
```sh
qr-code [https://hub.docker.com/] [jpeg] [600]
```


## 📦 Dependencies
- 🌐 `qrcode` - for generating the code
- 📜 `serde` and `serde_json` - for parsing data type
- 📜 `tokio` for asynchronous runtime environment
- 📜 `image` for converting qrcode to image format



## 🛡️ Error Handling
- The CLI gracefully handles network errors and invalid arguments inputs.
- If an error occurs, it provides a meaningful error message.

## 📜 License
This project is licensed under the **MIT License**.

## 🤝 Contribution
Feel free to **fork** the repository, **create issues**, or **submit pull requests**!

--- 