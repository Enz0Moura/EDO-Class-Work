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

Modela a temperatura de um corpo ao longo do tempo quando ele troca calor com um ambiente de temperatura constante.

**Ideia física**: a taxa de resfriamento (ou aquecimento) é proporcional à diferença entre a temperatura do corpo e a temperatura do ambiente. Quanto mais distante do equilíbrio térmico (\(T \neq T_{amb}\)), mais rápida é a variação; quando \(T\) se aproxima de \(T_{amb}\), a variação diminui.

**Parâmetros do modelo** (ver `src/exercises/newton/cooling_law/parameters.rs`):

- \(T_{amb}\): temperatura do ambiente (`env_temperature`)
- \(T_0\): temperatura inicial (`initial_temperature`)
- \(k\): constante de resfriamento (`k`) — controla quão rápido o corpo tende ao equilíbrio

**Solução analítica**:

\[
T(t) = T_{amb} + (T_0 - T_{amb}) e^{-kt}
\]

**Forma diferencial** (para solvers numéricos):

\[
\frac{dT}{dt} = -k(T - T_{amb})
\]

No menu de Newton, há opções para:

- **Analytical**: usa a fórmula fechada para calcular \(T(t)\) em uma malha de tempos e gera um gráfico com múltiplos valores de `k`.
- **Euler**: resolve numericamente a EDO com passo fixo \(h\) e plota a evolução temporal aproximada.
- **Compare**: plota a curva analítica e, sobre ela, os pontos calculados por Euler, para visualizar o erro de discretização.

**Como interpretar os resultados**:

- A solução analítica é a referência.
- O método de Euler é uma aproximação de primeira ordem: reduzir o passo \(h\) geralmente melhora a aproximação (ao custo de mais iterações).
- Para `k` maiores, a dinâmica é mais “rápida” (decaimento exponencial mais forte), e um passo \(h\) grande pode introduzir erro visível.

**Parâmetros usados nos experimentos (hard-coded em `src/exercises/mod.rs`)**:

- Intervalo de tempo típico: `t ∈ [0, 10]`
- Exemplo de ambiente: `T_amb = 20°C`
- Exemplo de condição inicial: `T0 = 90°C`
- No gráfico multi-`k`: `k ∈ {0.05, 0.10, 0.20, 0.50}`
- Euler (Newton): passo `h = 0.1`

### Problema 3 — Modelo Logístico

Modelo clássico de **crescimento limitado** (com capacidade de suporte normalizada em 1).

**Interpretação**: uma população (ou fração de ocupação) cresce aproximadamente de forma exponencial quando pequena, mas desacelera conforme se aproxima do limite superior, produzindo uma curva em “S” (sigmoide).

**Parâmetro principal**:

- \(y_0\): condição inicial (`LogisticParams { y0 }`).

No código, o modelo está normalizado para que o equilíbrio estável seja próximo de \(y=1\).

**Forma diferencial**:

\[
\frac{dy}{dt} = y(1-y)
\]

**Solução analítica** usada no projeto:

\[
y(t) = \frac{1}{1 + a e^{-t}}, \quad a = \frac{1-y_0}{y_0}
\]

No menu Logístico, há opções para:

- **Generate Data**:
  - gera amostras \((t_i, y_i)\) a partir do modelo analítico;
  - adiciona ruído Gaussiano (Normal) com desvio padrão `noise_std`;
  - plota: curva “limpa” (vermelho) vs. pontos ruidosos (azul).
- **Euler**:
  - resolve a EDO logística com Euler em uma malha de tempo;
  - plota pontos obtidos por Euler e a curva analítica para comparação.
- **Learned Model**:
  - gera dados com ruído;
  - estima uma EDO aproximada do tipo
    \(\frac{dy}{dt} \approx a_0 + a_1 y + a_2 y^2\)
    (um polinômio em \(y\));
  - resolve essa EDO “aprendida” via Euler e compara com a solução analítica.
- **Learned Model No Noise**:
  - repete o processo anterior, mas com dados sem ruído;
  - serve como baseline para observar o impacto do ruído nos coeficientes aprendidos.

**Como interpretar os resultados**:

- Em **Generate Data**, quanto maior `noise_std`, mais os pontos se afastam da curva analítica.
- Em **Euler**, a diferença entre pontos (Euler) e curva (analítica) evidencia o erro numérico do método.
- Em **Learned Model**:
  - os coeficientes \(a_0,a_1,a_2\) são impressos no terminal;
  - com ruído, o modelo ajustado tende a “entortar” a dinâmica (erro de identificação), e o gráfico pode se descolar da curva analítica;
  - sem ruído, a tendência é o ajuste se aproximar mais do comportamento real.

**Parâmetros usados nos experimentos (hard-coded em `src/exercises/mod.rs`)**:

- `t ∈ [0, 10]`, `n = 50` amostras
- condição inicial típica: `y0 = 0.1`
- Euler (logístico): passo `h = 0.2`
- ruído (dados sintéticos): `noise_std = 0.1` (ou `0.0` no modo sem ruído)

## Solver numérico: Método de Euler

O solver está em `src/solvers/euler/` e opera sobre qualquer tipo que implemente:

```rust
pub trait DifferentialEquation {
    fn derivative(&self, t: f64, y: f64) -> f64;
}
```

O Euler gera uma sequência de estados `(t, y)` via um iterador.

### Observações sobre o passo `h`

O método de Euler usado aqui é:

\[
y_{n+1} = y_n + h\,f(t_n, y_n)
\]

onde `f(t, y)` é a derivada fornecida pelo modelo (`derivative`).

- Passos menores (\(h\) menor) tendem a reduzir o erro numérico.
- Passos maiores podem ser suficientes para visualização qualitativa, mas podem piorar a aproximação.

## Mínimos quadrados (aprendizado do modelo)

Para o Problema 3, o código monta um sistema do tipo:

- matriz `A` com linhas `[1, y_i, y_i^2]`
- vetor `r` com aproximação de derivada `(y_{i+1} - y_i)/h`

e resolve `A * x ≈ r` por SVD (`nalgebra`). O vetor `x = [a0, a1, a2]` define o `LearnedModel`.

### O que está sendo ajustado, exatamente?

O projeto não ajusta diretamente \(y(t)\). Ele ajusta a **dinâmica** (a derivada) aproximando:

\[
\frac{dy}{dt} \approx a_0 + a_1 y + a_2 y^2
\]

com base em uma derivada estimada por diferenças finitas:

\[
\frac{dy}{dt}(t_i) \approx \frac{y_{i+1} - y_i}{h}
\]

Depois, essa EDO aprendida é resolvida novamente com Euler.

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
