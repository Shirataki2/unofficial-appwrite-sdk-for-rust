use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, SmartDefault, Display)]
pub enum Permission {
    #[default]
    #[display(fmt = "role:all")]
    Anyone,
    #[display(fmt = "role:guest")]
    Guest,
    #[display(fmt = "role:member")]
    Logined,
    #[display(fmt = "user:{_0}")]
    User(String),
    #[display(fmt = "team:{_0}")]
    Team(String),
    #[display(fmt = "team:{}/{}", "_0.0", "_0.1")]
    Role((String, String)),
    #[display(fmt = "member:{_0}")]
    Member(String),
}

pub trait PermissionExt {
    fn to_form_payload(&self) -> String;
}

impl PermissionExt for &[Permission] {
    fn to_form_payload(&self) -> String {
        self.iter()
            .filter_map(|p| serde_json::to_string(p).ok())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl serde::Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Permission::Anyone => serializer.serialize_str("role:all"),
            Permission::Guest => serializer.serialize_str("role:guest"),
            Permission::Logined => serializer.serialize_str("role:member"),
            Permission::User(user_id) => serializer.serialize_str(&format!("user:{}", user_id)),
            Permission::Team(team_id) => serializer.serialize_str(&format!("team:{}", team_id)),
            Permission::Role((team_id, role)) => {
                serializer.serialize_str(&format!("team:{}/{}", team_id, role))
            }
            Permission::Member(membership_id) => {
                serializer.serialize_str(&format!("member:{}", membership_id))
            }
        }
    }
}

struct PermissionVisitor;

impl<'de> serde::de::Visitor<'de> for PermissionVisitor {
    type Value = Permission;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a permission")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match s {
            "role:all" => Ok(Permission::Anyone),
            "role:guest" => Ok(Permission::Guest),
            "role:member" => Ok(Permission::Logined),
            _ => {
                let s = s.replace('/', ":");
                let parts: Vec<&str> = s.split(':').collect();
                match parts.as_slice() {
                    ["user", user_id] => Ok(Permission::User(user_id.to_string())),
                    ["team", team_id] => Ok(Permission::Team(team_id.to_string())),
                    ["team", team_id, role] => {
                        Ok(Permission::Role((team_id.to_string(), role.to_string())))
                    }
                    ["member", membership_id] => Ok(Permission::Member(membership_id.to_string())),
                    _ => Err(E::custom(format!("invalid permission: {}", s))),
                }
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PermissionVisitor)
    }
}

#[test]
fn test_serialize() {
    let permissions = vec![
        Permission::Anyone,
        Permission::Guest,
        Permission::Logined,
        Permission::User("1".to_string()),
        Permission::Team("2".to_string()),
        Permission::Role(("3".to_string(), "4".to_string())),
        Permission::Member("5".to_string()),
    ];

    let string_perms = vec![
        "\"role:all\"",
        "\"role:guest\"",
        "\"role:member\"",
        "\"user:1\"",
        "\"team:2\"",
        "\"team:3/4\"",
        "\"member:5\"",
    ];

    for (permission, expected) in permissions.iter().zip(string_perms.iter()) {
        let serialized = serde_json::to_string(permission).unwrap();
        assert_eq!(serialized, *expected);
    }

    for (expected, string_perm) in string_perms.iter().zip(permissions.iter()) {
        let deserialized: Permission = serde_json::from_str(expected).unwrap();
        assert_eq!(deserialized, *string_perm);
    }
}
