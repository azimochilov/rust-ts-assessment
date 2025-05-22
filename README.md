# ✍️ AI Paraphraser App

A full-stack AI-powered paraphrasing tool built using a **TypeScript + Vite** frontend and a **Rust + Shuttle** backend. The user can input and select text, which is then sent to an OpenAI API to generate a paraphrased version.

This project was built for a technical assessment.

---

## 🔗 Live Application

👉 **Frontend**: [https://rust-ts-assessment.vercel.app](https://rust-ts-assessment.vercel.app)  
👉 **Backend**: [https://ai-paraphraser-backend-pnp9.shuttle.app/paraphrase](https://ai-paraphraser-backend-pnp9.shuttle.app/paraphrase)

---

## 🧠 Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Axum + Shuttle
- **AI Provider**: OpenAI (GPT model)
- **Deployment**:
  - Frontend → [Vercel](https://vercel.com)
  - Backend → [Shuttle.dev](https://shuttle.rs)

---

## 🛠 Local Setup Instructions

### ✅ Prerequisites

- Node.js (v14 or higher)
- npm (comes with Node.js)
- Rust (latest stable)
- Cargo (comes with Rust)
- [Shuttle CLI](https://docs.shuttle.rs/introduction/installation) (`cargo install shuttle-launcher`)
- An OpenAI API key

---

### 📦 1. Clone the Repository

```bash
git clone https://github.com/azimochilov/rust-ts-assessment.git
cd rust-ts-assessment
