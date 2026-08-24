use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Investimento {
    pub id: i32,
    pub nome: String,
    pub tipo: String,
    pub valor: f64,
}

#[derive(Deserialize)]
pub struct CriarInvestimento {
    pub nome: String,
    pub tipo: String,
    pub valor: f64,
}

pub async fn health_check() -> &'static str {
    "API da Carteira de Investimentos rodando com sucesso!"
}

pub async fn listar_investimentos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Investimento>>, StatusCode> {
    let investimentos =
        sqlx::query_as::<_, Investimento>("SELECT id, nome, tipo, valor FROM investimentos")
            .fetch_all(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(investimentos))
}

pub async fn criar_investimento(
    State(pool): State<PgPool>,
    Json(payload): Json<CriarInvestimento>,
) -> Result<(StatusCode, Json<Investimento>), (StatusCode, String)> {
    if payload.nome.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "O nome do investimento é obrigatório.".into(),
        ));
    }
    if payload.valor <= 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "O valor deve ser maior que zero.".into(),
        ));
    }

    let registro = sqlx::query_as::<_, Investimento>(
        "INSERT INTO investimentos (nome, tipo, valor) VALUES ($1, $2, $3) RETURNING id, nome, tipo, valor",
    )
    .bind(&payload.nome)
    .bind(&payload.tipo)
    .bind(payload.valor)
    .fetch_one(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao salvar no banco de dados.".into()))?;

    Ok((StatusCode::CREATED, Json(registro)))
}

pub async fn valor_total(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let total: Option<f64> = sqlx::query_scalar("SELECT SUM(valor) FROM investimentos")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "valor_total": total.unwrap_or(0.0)
    })))
}
