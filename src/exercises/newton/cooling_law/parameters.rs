use std::fmt;
/// Estrutura que representa os **parâmetros físicos** da Lei de
/// Resfriamento de Newton.
///
/// Esta struct contém apenas os **dados necessários para definir o modelo
/// físico**, sendo reutilizada pelas implementações:
///
/// - `CoolingAnalytical` → solução analítica da equação
/// - `CoolingDifferential` → equação diferencial utilizada por solvers numéricos
///
/// Separar os parâmetros do modelo das implementações permite:
///
/// - reutilização entre diferentes métodos de solução
/// - redução de duplicação de código
/// - maior clareza conceitual entre **modelo físico** e **método de resolução**
///
/// ## Parâmetros do modelo
///
/// A Lei de Resfriamento de Newton depende de três grandezas principais:
///
/// - `T_amb` → temperatura do ambiente
/// - `T0` → temperatura inicial do objeto
/// - `k` → constante de resfriamento
///
/// Esses valores são utilizados tanto na solução analítica quanto na forma
/// diferencial da equação.
///
/// ## Escolha dos tipos
///
/// ### `f64`
///
/// Utilizado para representar temperaturas e constantes do modelo.
///
/// Intervalo aproximado:
///
/// ±1.7976931348623157 × 10^308
///
///
/// Precisão:
///
/// ~15–17 dígitos significativos
///
///
/// ### `Option<f64>`
///
/// A constante de resfriamento `k` pode não ser conhecida no momento da
/// criação do modelo.
///
/// O uso de `Option` permite representar explicitamente essa ausência
/// de valor.
///
/// Isso torna o estado do modelo **mais seguro e explícito em tempo de
/// compilação**.

#[derive(Clone)]
pub struct CoolingParams {
    /// Temperatura do ambiente (`T_amb`)
    ///
    /// Tipo: `f64`
    ///
    /// Representa a temperatura do meio em que o objeto está inserido.
    /// O processo de resfriamento ocorre em direção a esse valor.
    pub env_temperature: f64,

    /// Temperatura inicial do objeto (`T0`)
    ///
    /// Tipo: `f64`
    ///
    /// Temperatura do objeto no instante inicial `t = 0`.
    /// Este valor define a condição inicial do modelo físico.
    pub initial_temperature: f64,

    /// Constante de resfriamento (`k`)
    ///
    /// Tipo: `Option<f64>`
    ///
    /// Representa a taxa de troca térmica entre o objeto e o ambiente.
    ///
    /// - valores maiores indicam resfriamento mais rápido
    /// - valores menores indicam resfriamento mais lento
    ///
    /// Pode ser indefinida inicialmente, sendo configurada posteriormente.
    pub k: Option<f64>,
}

impl CoolingParams {
    /// Retorna o valor da constante de resfriamento `k`.
    ///
    /// # Panics
    ///
    /// Este método gera panic caso `k` ainda não tenha sido definido.
    ///
    /// Isso evita cálculos inválidos quando o modelo ainda não possui
    /// todos os parâmetros necessários.
    pub fn get_k(&self) -> f64 {
        self.k.expect("K must be defined")
    }

    /// Define o valor da constante de resfriamento `k`.
    ///
    /// Este método permite configurar o parâmetro após a criação
    /// da struct.
    ///
    /// # Parâmetros
    ///
    /// - `k` → constante de resfriamento do modelo
    pub fn set_k(&mut self, k: f64) {
        self.k = Some(k);
    }
}

impl fmt::Display for CoolingParams {
    //Implementação do print de display para a struct
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cooling Law Parameters")?;
        writeln!(f, "Environment Temperature: {}", self.env_temperature)?;
        writeln!(f, "Initial Temperature: {}", self.initial_temperature)?;

        match self.k {
            Some(k) => writeln!(f, "k: {}", k)?,
            None => writeln!(f, "k: not defined")?,
        }

        Ok(())
    }
}
