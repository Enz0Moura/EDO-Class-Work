# Trabalho Edo

Trabalho de **EDO (Equações Diferenciais Ordinárias)** implementado em **Rust**, contendo:

- modelagens (forma **analítica** e **diferencial**) para problemas clássicos;
- resolução numérica via **Método de Euler**;
- geração de gráficos (`.png`) com **plotters**;
- geração de dados sintéticos com ruído e ajuste de um modelo via **Mínimos Quadrados**.

O binário é **interativo**: ao executar, você navega por um menu no terminal para rodar cada experimento.

## Especificações do projeto

### Linguagem / ferramenta

- Rust (edition **2024**)
- Projeto Cargo: `trabalho_edo` (v1.0.1)

### Dependências

- [`plotters`](https://crates.io/crates/plotters): geração de gráficos (PNG)
- [`rand`](https://crates.io/crates/rand) + [`rand_distr`](https://crates.io/crates/rand_distr): geração de ruído Gaussiano
- [`nalgebra`](https://crates.io/crates/nalgebra): álgebra linear (SVD) para mínimos quadrados

### Estrutura (alto nível)

- `src/cli/`: menu interativo do terminal.
- `src/exercises/`: exercícios ("Problem 1" e "Problem 3").
- `src/solvers/`:
  - `euler/`: solver numérico (Euler) para EDOs.
  - `differetials/`: trait `DifferentialEquation` (interface para EDOs).
  - `data/`: geração de dados sintéticos (com ruído).
  - `least_squares/`: construção e solução do problema de mínimos quadrados.
- `src/utils/`:
  - `linspace/`: geração de malha linear de tempo.
  - `experiment_path/`: padroniza e cria pastas/paths de saída em `outputs/`.

## Exercícios implementados

### Problema 1 — Lei de Resfriamento de Newton

Modela a temperatura de um corpo ao longo do tempo em contato com um ambiente.

**Solução analítica**:

\[
T(t) = T_{amb} + (T_0 - T_{amb}) e^{-kt}
\]

**Forma diferencial** (para solvers numéricos):

\[
\frac{dT}{dt} = -k(T - T_{amb})
\]

No menu de Newton, há opções para:

- **Analytical**: gera um gráfico com múltiplos valores de `k`.
- **Euler**: resolve numericamente com Euler e plota evolução temporal.
- **Compare**: compara a solução analítica com a aproximação de Euler.

### Problema 3 — Modelo Logístico

Modelo clássico de crescimento limitado.

**Forma diferencial**:

\[
\frac{dy}{dt} = y(1-y)
\]

**Solução analítica** usada no projeto:

\[
y(t) = \frac{1}{1 + a e^{-t}}, \quad a = \frac{1-y_0}{y_0}
\]

No menu Logístico, há opções para:

- **Generate Data**: gera dados sintéticos com ruído Gaussiano e salva um gráfico.
- **Euler**: resolve a EDO logística com Euler e plota (também plota a curva analítica para comparação).
- **Learned Model**: ajusta um modelo do tipo
  \(\frac{dy}{dt} \approx a_0 + a_1 y + a_2 y^2\)
  via mínimos quadrados (SVD) usando dados com ruído e resolve com Euler.
- **Learned Model No Noise**: mesma ideia, mas com dados sem ruído.

## Solver numérico: Método de Euler

O solver está em `src/solvers/euler/` e opera sobre qualquer tipo que implemente:

```rust
pub trait DifferentialEquation {
    fn derivative(&self, t: f64, y: f64) -> f64;
}
```

O Euler gera uma sequência de estados `(t, y)` via um iterador.

## Mínimos quadrados (aprendizado do modelo)

Para o Problema 3, o código monta um sistema do tipo:

- matriz `A` com linhas `[1, y_i, y_i^2]`
- vetor `r` com aproximação de derivada `(y_{i+1} - y_i)/h`

e resolve `A * x ≈ r` por SVD (`nalgebra`). O vetor `x = [a0, a1, a2]` define o `LearnedModel`.

## Saídas (plots)

Os experimentos salvam imagens em `outputs/<problem>/<section>/...`.
O diretório `outputs/` está no `.gitignore`.

Arquivos gerados atualmente pelo código:

- `outputs/problem1/newton_analytical/analytical_multi_k.png`
- `outputs/problem1/euler_time/euler_time.png`
- `outputs/problem1/compare/compare.png`
- `outputs/problem3/experimental_data/experimental_data.png`
- `outputs/problem3/euler_logistic/euler.png`
- `outputs/problem3/learned_model/learned_model.png`
- `outputs/problem3/learned_model/learned_model_no_noise.png`

## Como compilar e executar

### Requisitos

- Rust instalado (toolchain estável)

### Build

```bash
cargo build
```

### Executar

```bash
cargo run
```

Você verá um menu como:

```text
==== MAIN MENU ====
1 - Problem 1 (Newton)
3 - Problem 3 (Logistic)
0 - Exit
```

Selecione as opções para gerar os gráficos. Ao final, o programa imprime o caminho do arquivo salvo.

## CI

Há um workflow do GitHub Actions em `.github/workflows/rust.yml` que executa:

- `cargo build --verbose`
- `cargo test --verbose`
