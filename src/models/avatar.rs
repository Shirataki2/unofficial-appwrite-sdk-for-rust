use std::ops::Deref;

use crate::services::avatars::AvatarService;

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

pub struct Avatar {
    service: AvatarService,
}

impl Avatar {
    pub fn new() -> Self {
        Avatar {
            service: AvatarService {},
        }
    }
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Avatar {
    type Target = AvatarService;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}
