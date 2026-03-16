use crate::exercises::newton::cooling_law::parameters::CoolingParams;
use std::fmt;
/// Modelo analítico da **Lei de Resfriamento de Newton**.
///
/// Este módulo implementa a **solução fechada (analítica)** da equação
/// diferencial que descreve o resfriamento de um corpo ao longo do tempo.
///
/// A temperatura do objeto é dada por:
///
/// T(t) = T_{amb} + (T_0 - T_{amb}) e^{-kt}
///
/// Onde:
///
/// - `T(t)` → temperatura do objeto no tempo `t`
/// - `T_amb` → temperatura do ambiente
/// - `T0` → temperatura inicial do objeto
/// - `k` → constante de resfriamento
/// - `t` → tempo
///
/// ## Interpretação Física
///
/// A Lei de Resfriamento de Newton afirma que:
///
/// > A taxa de variação da temperatura de um corpo é proporcional
/// > à diferença entre sua temperatura e a temperatura do ambiente.
///
/// A solução analítica permite calcular diretamente a temperatura
/// para qualquer instante `t`, sem a necessidade de métodos numéricos.
///
/// ## Responsabilidade deste módulo
///
/// Este struct representa **a solução analítica do modelo**, permitindo:
///
/// - calcular `T(t)` diretamente
/// - armazenar o último valor calculado
/// - reutilizar os parâmetros físicos definidos em `CoolingParams`
///
/// ## Escolha dos tipos
///
/// - `f64` é utilizado para representar temperaturas e constantes.
///
/// Intervalo aproximado:
///
/// ```text
/// ±1.7976931348623157 × 10^308
/// ```
///
/// Precisão:
///
/// ```text
/// ~15–17 dígitos significativos
/// ```
///
/// Justificativa:
///
/// - A função exponencial `exp()` da biblioteca padrão opera em `f64`.
/// - Mantém precisão adequada em cálculos científicos.
/// - Evita conversões implícitas que poderiam impactar performance.
///
/// ## Uso de `Option`
///
/// O campo `temperature` utiliza `Option<f64>` porque:
///
/// - a temperatura só existe **após a avaliação da função**
/// - evita o uso de valores sentinela como `0` ou `NaN`
/// - torna o estado do modelo explicitamente seguro.
///
pub struct CoolingLaw {
    /// Parâmetros físicos do modelo
    ///
    /// Contém:
    ///
    /// - temperatura do ambiente
    /// - temperatura inicial
    /// - constante de resfriamento `k`
    params: CoolingParams,

    /// Última temperatura calculada `T(t)`
    ///
    /// Tipo: `Option<f64>`
    ///
    /// Justificativa:
    ///
    /// - o valor só existe após executar `temperature_at`
    /// - previne estados inconsistentes no modelo
    temperature: Option<f64>,
}

impl CoolingLaw {
    pub fn new(params: CoolingParams) -> Self {
        Self {
            params: params,
            temperature: None,
        }
    }

    pub fn with_k(mut self, k: f64) -> Self {
        self.params.k = Some(k);
        self
    }

    pub fn set_temperature(&mut self, temperature: f64) -> Option<f64> {
        self.temperature = Some(temperature);
        self.temperature
    }

    pub fn temperature_at(&mut self, time: f64) -> Option<f64> {
        let k = self.params.k?;

        let temp = self.params.env_temperature
            + (self.params.initial_temperature - self.params.env_temperature)
                * (-k * time).exp();

        self.set_temperature(temp)
    }

    pub fn with_temperature_at(mut self, time: f64) -> Self {
        let _ = self.temperature_at(time);
        self
    }

}

impl fmt::Display for CoolingLaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cooling Law Analytical Model")?;
        writeln!(f, "Environment Temperature: {}", self.params.env_temperature)?;
        writeln!(f, "Initial Temperature: {}", self.params.initial_temperature)?;

        match self.params.k {
            Some(k) => writeln!(f, "k: {}", k)?,
            None => writeln!(f, "k: not defined")?,
        }

        match self.temperature {
            Some(t) => writeln!(f, "Temperature: {}", t)?,
            None => writeln!(f, "Temperature: not calculated")?,
        }

        Ok(())
    }
}
