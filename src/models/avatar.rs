#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CreditcardCode {
    Amex,
    Argencard,
    Cabal,
    Cencosud,
    Diners,
    Discover,
    Elo,
    Hipercard,
    Jcb,
    Maestro,
    Mastercard,
    Mir,
    Naranja,
    TarjetaShopping,
    UnionChinaPay,
    Visa,
}
