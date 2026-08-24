# Carteira de Investimentos - API REST em Rust

API para gerenciamento e consolidação de carteiras de investimentos desenvolvida em Rust.

## 🚀 Funcionalidades

- **`GET /`**: Verificação de status da API
- **`GET /investimentos`**: Listagem de todos os ativos
- **`POST /investimentos`**: Cadastro de novos ativos com validação de campos
- **`GET /investimentos/total`**: Consolidação do valor total da carteira

## 🛠️ Tecnologias Utilizadas

- **Rust** (Edição 2021)
- **Axum** (Framework Web)
- **SQLx** (Driver de Banco de Dados Assíncrono)
- **PostgreSQL / Docker**

## 💻 Como Rodar o Projeto

1. Inicie o banco de dados via Docker:
   ```bash
   docker compose up -d