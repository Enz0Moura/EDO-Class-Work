/// Implementação da **Lei de Resfriamento de Newton**.
///
/// Este módulo contém uma modelagem computacional do processo de
/// resfriamento de um corpo em contato com um ambiente.
///
/// A Lei de Resfriamento de Newton descreve como a temperatura de um
/// objeto varia ao longo do tempo quando há troca de calor com o meio
/// ao redor.
///
/// A solução analítica da equação é dada por:
///
/// ```text
/// T(t) = T_amb + (T0 − T_amb) e^(−k t)
/// ```
///
/// onde:
///
/// - `T(t)` → temperatura do objeto no tempo `t`
/// - `T_amb` → temperatura do ambiente
/// - `T0` → temperatura inicial do objeto
/// - `k` → constante de resfriamento
///
/// ## Organização do módulo
///
/// A implementação foi dividida em três componentes principais para
/// separar **dados físicos**, **modelos matemáticos** e **métodos de
/// resolução**.
///
/// ```text
/// cooling_law
/// ├── parameters   → parâmetros físicos do modelo
/// ├── analytical   → solução analítica da equação
/// └── differential → equação diferencial para solvers numéricos
/// ```
///
/// ### `parameters`
///
/// Define a estrutura `CoolingParams`, responsável por armazenar
/// os parâmetros físicos do modelo:
///
/// - temperatura do ambiente
/// - temperatura inicial
/// - constante de resfriamento `k`
///
/// Esses parâmetros são compartilhados entre as diferentes
/// representações matemáticas do modelo.
///
/// ### `analytical`
///
/// Implementa a solução **analítica** da equação diferencial,
/// permitindo calcular diretamente a temperatura para um tempo `t`.
///
/// Esse método utiliza a forma fechada da equação e não depende
/// de métodos numéricos.
///
/// ### `differential`
///
/// Define a forma **diferencial** da Lei de Resfriamento de Newton:
///
/// ```text
/// dT/dt = -k(T - T_amb)
/// ```
///
/// Essa representação é utilizada por **solvers numéricos**, como
/// o método de Euler implementado no módulo `solvers`.
///
/// ## Objetivo da separação
///
/// Essa organização permite:
///
/// - reutilização dos mesmos parâmetros físicos
/// - uso do modelo com diferentes métodos numéricos
/// - comparação entre solução analítica e aproximações numéricas
/// - maior clareza conceitual entre **modelo físico** e **método
///   de resolução**
///
pub mod analytical;
pub mod differential;
pub mod parameters;
