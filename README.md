# Trabalho EDO

Trabalho de **Equações Diferenciais Ordinárias (EDO)** implementado em **Rust**. O projeto reúne:

- modelagem **analítica** (solução fechada) e **diferencial** (EDO) de problemas clássicos;
- resolução numérica via **Método de Euler**;
- geração de gráficos `.png` com **plotters**;
- geração de dados sintéticos com **ruído Gaussiano** e ajuste de um modelo via **Mínimos Quadrados**.

O programa é um **binário interativo**: ao executar, você navega por um menu no terminal para rodar os experimentos.

---

## Quickstart

```bash
# build
cargo build

# run (menu interativo)
cargo run
```

Os gráficos são salvos dentro de `outputs/` (diretório ignorado pelo git).

---

## Especificações / stack

- Rust (edition **2024**)
- `plotters`: geração de gráficos (PNG)
- `rand` + `rand_distr`: ruído Gaussiano (Normal)
- `nalgebra`: álgebra linear (SVD) para mínimos quadrados

---

## Como usar (mapa do menu)

Ao executar `cargo run`, você verá:

```text
==== MAIN MENU ====
1 - Problem 1 (Newton)
3 - Problem 3 (Logistic)
0 - Exit
```

### Problem 1 (Newton)

```text
==== PROBLEM 1: NEWTON ====
1 - Analytical
2 - Euler
3 - Compare
0 - Back
```

### Problem 3 (Logistic)

```text
==== PROBLEM 3: LOGISTIC ====
1 - Generate Data
2 - Euler
3 - Learned Model
4 - Learned Model No Noise
0 - Back
```

---

## Estrutura do código (alto nível)

- `src/cli/`: menu interativo.
- `src/exercises/`: implementação dos problemas/experimentos.
- `src/solvers/`:
  - `differetials/`: trait `DifferentialEquation`.
  - `euler/`: método de Euler + iterador de estados.
  - `data/`: gerador de dados sintéticos com ruído.
  - `least_squares/`: construção e solução do problema de mínimos quadrados.
- `src/utils/`:
  - `linspace/`: malha linear de tempo.
  - `experiment_path/`: cria paths em `outputs/<problem>/<section>/...`.

---

## Problema 1 — Lei de Resfriamento de Newton

### Objetivo

Modelar a temperatura $T(t)$ de um corpo ao longo do tempo em contato com um ambiente a temperatura constante $T_{\mathrm{amb}}$.

### Hipótese física

A **taxa de variação** da temperatura do corpo é proporcional à diferença para o ambiente:

- se $T(t) > T_{\mathrm{amb}}$, o corpo esfria;
- se $T(t) > T_{\mathrm{amb}}$, o corpo aquece;
- quanto maior $T(t) > T_{\mathrm{amb}}$, maior a taxa de variação.

### Parâmetros (ver `src/exercises/newton/cooling_law/parameters.rs`)

- $T_{amb}$  : temperatura do ambiente (`env_temperature`)
- $T_0$ : temperatura inicial (`initial_temperature`)
- $k > 0$ : constante de resfriamento (`k`) — controla a taxa de decaimento

### Modelo

**Forma diferencial**:

- $T_{amb}$ : temperatura do ambiente (`env_temperature`)
- $T_0$ : temperatura inicial (`initial_temperature`)
- $k > 0$ : constante de resfriamento (`k`) — controla a taxa de decaimento



**Solução analítica**:

\[
T(t) = T_{amb} + (T_0 - T_{amb})e^{-kt}
\]

### Experimentos (o que cada opção faz)

Implementados em `src/exercises/mod.rs`:

- **Analytical**: calcula \(T(t)\) para uma malha de tempo e plota curvas para múltiplos valores de `k`.
- **Euler**: resolve a EDO com Euler (passo fixo \(h\)) e plota a evolução temporal aproximada.
- **Compare**: plota a curva analítica e sobrepõe os pontos do Euler para visualizar o erro.

### Como interpretar

- A curva **analítica** é a referência (exata dentro do modelo).
- O Euler é **1ª ordem**: passos menores tendem a aproximar melhor a curva.
- Para dinâmicas mais rápidas (k maior), geralmente é necessário reduzir \(h\) para manter boa aproximação.

---

## Problema 3 — Modelo Logístico

### Objetivo

Modelar uma variável \(y(t)\) com **crescimento limitado** (capacidade de suporte normalizada em 1), típica de curvas sigmoides.

### Interpretação

- para \(y\) pequeno, o crescimento é quase exponencial;
- conforme \(y\to 1\), o crescimento desacelera e tende ao equilíbrio.

### Parâmetro

- \(y_0\): condição inicial (`LogisticParams { y0 }`).

### Modelo

**Forma diferencial**:

\[
\frac{dy}{dt} = y(1-y)
\]

**Solução analítica** (usada para referência e geração de dados):

\[
y(t)=\frac{1}{1 + a e^{-t}},\quad a=\frac{1-y_0}{y_0}
\]

### Experimentos (o que cada opção faz)

- **Generate Data**:
  - gera amostras \((t_i, y_i)\) a partir da solução analítica;
  - adiciona ruído Normal com desvio padrão `noise_std`;
  - plota curva limpa vs. pontos ruidosos.
- **Euler**:
  - resolve \(dy/dt=y(1-y)\) com Euler;
  - plota pontos do Euler e a curva analítica para comparação.
- **Learned Model**:
  - gera dados com ruído;
  - estima a dinâmica por mínimos quadrados na forma:
    \(\frac{dy}{dt}\approx a_0 + a_1 y + a_2 y^2\);
  - resolve a EDO aprendida via Euler e compara com a curva analítica.
- **Learned Model No Noise**: igual ao anterior, mas com `noise_std = 0.0`.

### Como interpretar

- Mais ruído ⇒ maior dispersão nos pontos e maior dificuldade em recuperar a dinâmica verdadeira.
- O “modelo aprendido” aproxima a **derivada** (dinâmica), não diretamente \(y(t)\).
- Os coeficientes \(a_0,a_1,a_2\) são impressos no terminal; compare o comportamento com e sem ruído.

---

## Solver numérico — Método de Euler

O solver opera sobre qualquer tipo que implemente:

```rust
pub trait DifferentialEquation {
    fn derivative(&self, t: f64, y: f64) -> f64;
}
```

Fórmula usada:

\[
y_{n+1} = y_n + h\,f(t_n, y_n)
\]

Observação: reduzir o passo \(h\) tende a reduzir o erro (mais iterações).

---

## Mínimos Quadrados (aprendizado da dinâmica)

Para os dados \((t_i, y_i)\), o código estima a derivada por diferenças finitas:

\[
r_i \approx \frac{y_{i+1} - y_i}{h}
\]

e monta a matriz com linhas:

\[
\mathbf{A}_i = [1,\; y_i,\; y_i^2]
\]

Resolvendo \(Ax\approx r\) por SVD (`nalgebra`), obtém-se \(x=[a_0,a_1,a_2]\), que define a EDO aprendida.

---

## Outputs

Os arquivos são gerados em:

```text
outputs/<problem>/<section>/<file>.png
```

Tabela de saídas (nomes atuais no código):

| Experimento | Arquivo gerado |
|---|---|
| Newton / Analytical | `outputs/problem1/newton_analytical/analytical_multi_k.png` |
| Newton / Euler | `outputs/problem1/euler_time/euler_time.png` |
| Newton / Compare | `outputs/problem1/compare/compare.png` |
| Logistic / Generate Data | `outputs/problem3/experimental_data/experimental_data.png` |
| Logistic / Euler | `outputs/problem3/euler_logistic/euler.png` |
| Logistic / Learned Model | `outputs/problem3/learned_model/learned_model.png` |
| Logistic / Learned Model No Noise | `outputs/problem3/learned_model/learned_model_no_noise.png` |

---

## Reprodutibilidade / Como alterar parâmetros

Os parâmetros dos experimentos estão, em geral, **hard-coded** em `src/exercises/mod.rs`. Exemplos:

- **Passo do Euler**:
  - Newton: `Euler::new(&equation, 0.1)`
  - Logístico: `Euler::new(&equation, 0.2)`
- **Intervalo de tempo** e quantidade de pontos: `t_min`, `t_max`, `n`.
- **Ruído** nos dados sintéticos: `generator.generate(&t_values, 0.1)`.
- **Condição inicial** (ex.: `y0 = 0.1`, `T0 = 90.0`).

Se você quiser transformar isso em parâmetros de linha de comando (ex.: `--step`, `--noise`), hoje não há parsing de args — o programa é guiado por menu.

---

## CI

O workflow `.github/workflows/rust.yml` roda:

- `cargo build --verbose`
- `cargo test --verbose`
