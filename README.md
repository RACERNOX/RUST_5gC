# RUST_5gC - AI Assistant Playground

A high-performance AI chat interface built with **Rust** and **Axum**. This project serves as a playground for integrating Large Language Models (LLMs) into a Rust backend, supporting both local models (via Ollama) and cloud-based models (Google Gemini).

## 🚀 Features

-   **Rust Backend**: Built on the robust `axum` web framework and `tokio` runtime.
-   **Multi-Model Support**:
    -   **Local LLM**: Integrate with local models like Llama 3 via Ollama.
    -   **Cloud LLM**: Native integration with Google Gemini Pro / Flash.
-   **Web Interface**: Clean, responsive chat UI using `Askama` templates and vanilla CSS.
-   **Streaming Ready**: Logic structure allows for easy extension to streaming responses (current implementation handles unary responses).

## 🛠️ Prerequisites

-   **Rust**: Ensure you have Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install)
-   **Ollama (Optional)**: If you plan to use local models. [Download Ollama](https://ollama.com/)
-   **Gemini API Key (Optional)**: If you plan to use Google Gemini. [Get API Key](https://aistudio.google.com/app/apikey)

## 📦 Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/RACERNOX/RUST_5gC.git
    cd RUST_5gC
    ```

2.  **Setup Environment Variables:**
    Copy the example environment file and configure your keys.
    ```bash
    cp .env.bak .env
    ```

## ⚙️ Configuration

Edit the `.env` file to choose your preferred model provider.

### Option A: Use Local LLM (Ollama)
Ensure Ollama is running (`ollama serve`).

```ini
# .env
CURRENT_MODEL=local
LOCAL_LLM_URL=http://localhost:11434/api/generate
LOCAL_MODEL=llama3
```

### Option B: Use Google Gemini
```ini
# .env
CURRENT_MODEL=gemini
GEMINI_API_KEY=your_actual_api_key_here
GEMINI_MODEL=gemini-1.5-flash-latest
```

## ▶️ Usage

1.  **Run the server:**
    ```bash
    cargo run
    ```

2.  **Access the application:**
    Open your browser and navigate to:
    -   **Chat Interface**: [http://127.0.0.1:3000/assistant/chat](http://127.0.0.1:3000/assistant/chat)
    -   **Info Page**: [http://127.0.0.1:3000/assistant/info](http://127.0.0.1:3000/assistant/info)

## 📂 Project Structure

-   `src/main.rs`: Entry point and server configuration.
-   `src/handlers.rs`: HTTP request handlers and routing logic.
-   `src/model_manager.rs`: Abstraction layer for LLM providers (Gemini/Local).
-   `templates/`: HTML templates using Askama.
-   `assets/`: Static assets (CSS, JS, Images).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open-source and available under the standard MIT License.
