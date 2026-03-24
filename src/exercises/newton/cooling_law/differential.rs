use crate::exercises::newton::cooling_law::parameters::CoolingParams;
use crate::solvers::differetials;
use std::fmt;

/// Representação **diferencial** da Lei de Resfriamento de Newton.
///
/// Este módulo define a **equação diferencial** que descreve a variação
/// da temperatura de um corpo ao longo do tempo.
///
/// A equação diferencial do modelo é:
///
/// dT/dt = -k(T - T_{amb})
///
/// Onde:
///
/// - `T` → temperatura do objeto
/// - `T_amb` → temperatura do ambiente
/// - `k` → constante de resfriamento
///
/// ## Interpretação Física
///
/// A Lei de Resfriamento de Newton estabelece que:
///
/// > A taxa de variação da temperatura de um objeto é proporcional
/// à diferença entre sua temperatura e a temperatura do ambiente.
///
/// Isso significa que:
///
/// - quanto maior a diferença de temperatura, mais rápido ocorre o resfriamento
/// - conforme o objeto se aproxima da temperatura ambiente, o resfriamento desacelera
///
/// ## Responsabilidade deste módulo
///
/// Este struct fornece a forma **diferencial do modelo**, permitindo que
/// métodos numéricos resolvam a equação ao longo do tempo.
///
/// Ele é utilizado por **solvers numéricos**, como:
///
/// - Método de Euler
///
/// A implementação do trait `DifferentialEquation` permite que o modelo
/// seja passado diretamente para um solver.
///
#[derive(Clone)]
pub struct CoolingDifferential {
    /// Parâmetros físicos do modelo
    ///
    /// Contém:
    ///
    /// - temperatura do ambiente (`T_amb`)
    /// - temperatura inicial (`T0`)
    /// - constante de resfriamento (`k`)
    ///
    /// Esses parâmetros são compartilhados entre as implementações
    /// analítica e diferencial do modelo.
    params: CoolingParams,
}

impl CoolingDifferential {
    /// Cria uma nova representação diferencial da Lei de Resfriamento.
    ///
    /// Recebe os parâmetros físicos do modelo definidos em `CoolingParams`.
    pub fn new(params: CoolingParams) -> Self {
        Self { params }
    }

    pub fn get_params(&self) -> &CoolingParams {
        &self.params
    }
}

impl differetials::DifferentialEquation for CoolingDifferential {
    /// Calcula a derivada da temperatura em relação ao tempo.
    ///
    /// Este método implementa diretamente a equação diferencial do modelo:
    ///
    /// ```text
    /// dT/dt = -k(T − T_amb)
    /// ```
    ///
    /// Parâmetros:
    ///
    /// - `_t` → tempo atual (não utilizado neste modelo específico)
    /// - `y` → temperatura atual do objeto
    ///
    /// Retorna:
    ///
    /// - taxa de variação da temperatura no instante atual
    fn derivative(&self, _t: f64, y: f64) -> f64 {
        -self.params.get_k() * (y - self.params.env_temperature)
    }
}

impl fmt::Display for CoolingDifferential {
    // Implementação do print da struct de CoolingDifferential
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cooling Law Differential Model")?;
        writeln!(
            f,
            "Environment Temperature: {}",
            self.params.env_temperature
        )?;
        writeln!(
            f,
            "Initial Temperature: {}",
            self.params.initial_temperature
        )?;

        match self.params.k {
            Some(k) => writeln!(f, "k: {}", k)?,
            None => writeln!(f, "k: not defined")?,
        }

        writeln!(f, "Differential Equation: dT/dt = -k(T - T_amb)")?;

        Ok(())
    }
}
