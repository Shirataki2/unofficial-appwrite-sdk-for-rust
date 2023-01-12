use std::{fmt, str::FromStr};

use crate::prelude::{MembershipId, TeamId, UserId};

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Permission {
    #[display(fmt = "read(\"{_0}\")")]
    Read(Role),
    #[display(fmt = "write(\"{_0}\")")]
    Write(Role),
    #[display(fmt = "create(\"{_0}\")")]
    Create(Role),
    #[display(fmt = "delete(\"{_0}\")")]
    Delete(Role),
    #[display(fmt = "update(\"{_0}\")")]
    Update(Role),
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum UserStatus {
    #[display(fmt = "verified")]
    Verified,
    #[display(fmt = "unverified")]
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Any,
    User((UserId, Option<UserStatus>)),
    Users(Option<UserStatus>),
    Guests,
    Team((TeamId, Option<Box<Role>>)),
    Member(MembershipId),
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Any => write!(f, "any"),
            Role::User((user_id, status)) => match status {
                Some(UserStatus::Verified) => write!(f, "user:{}/verified", user_id),
                Some(UserStatus::Unverified) => write!(f, "user:{}/unverified", user_id),
                None => write!(f, "user:{}", user_id),
            },
            Role::Users(status) => match status {
                Some(UserStatus::Verified) => write!(f, "users/verified"),
                Some(UserStatus::Unverified) => write!(f, "users/unverified"),
                None => write!(f, "users"),
            },
            Role::Guests => write!(f, "guests"),
            Role::Team((team_id, role)) => match role {
                Some(role) => write!(f, "team:{}/{}", team_id, role),
                None => write!(f, "team:{}", team_id),
            },
            Role::Member(membership_id) => write!(f, "member:{}", membership_id),
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ':');
        let role = parts.next().ok_or(())?;
        // remove double quotes
        let role = role.trim_matches('"');
        match role {
            "any" => Ok(Role::Any),
            "user" => {
                let id = parts.next().ok_or(())?;
                let mut parts = id.splitn(2, '/');
                let user_id = parts.next().ok_or(())?;
                let status = parts.next();
                match status {
                    Some(status) => {
                        let status = match status {
                            "verified" => Some(UserStatus::Verified),
                            "unverified" => Some(UserStatus::Unverified),
                            _ => None,
                        };
                        Ok(Role::User((UserId::new(user_id.into()), status)))
                    }
                    None => Ok(Role::User((UserId::new(user_id.into()), None))),
                }
            }
            "users" | "users/verified" | "users/unverified" => {
                let mut parts = role.splitn(2, '/');
                let _ = parts.next();
                let status = parts.next();
                match status {
                    Some(status) => {
                        let status = match status {
                            "verified" => Some(UserStatus::Verified),
                            "unverified" => Some(UserStatus::Unverified),
                            _ => None,
                        };
                        Ok(Role::Users(status))
                    }
                    None => Ok(Role::Users(None)),
                }
            }
            "guests" => Ok(Role::Guests),
            "team" => {
                let id = parts.next().ok_or(())?;
                let mut parts = id.splitn(2, '/');
                let team_id = parts.next().ok_or(())?;
                let role = parts.next();
                match role {
                    Some(role) => {
                        let role = role.parse().map_err(|_| ())?;
                        let team_id = TeamId::new(team_id.into());
                        Ok(Role::Team((team_id, Some(Box::new(role)))))
                    }
                    None => {
                        let team_id = TeamId::new(team_id.into());
                        Ok(Role::Team((team_id, None)))
                    }
                }
            }
            "member" => {
                let id = parts.next().ok_or(())?;
                let membership_id = MembershipId::new(id.into());
                Ok(Role::Member(membership_id))
            }
            _ => Err(()),
        }
    }
}

impl serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct RoleVisitor;

impl<'de> serde::de::Visitor<'de> for RoleVisitor {
    type Value = Role;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a role")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.parse().map_err(|_| E::custom("invalid role"))
    }
}

impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RoleVisitor)
    }
}

impl serde::Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Permission::Read(role) => serializer.serialize_str(&format!("read(\"{}\")", role)),
            Permission::Write(role) => serializer.serialize_str(&format!("write(\"{}\")", role)),
            Permission::Create(role) => serializer.serialize_str(&format!("create(\"{}\")", role)),
            Permission::Delete(role) => serializer.serialize_str(&format!("delete(\"{}\")", role)),
            Permission::Update(role) => serializer.serialize_str(&format!("update(\"{}\")", role)),
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
        let parts: Vec<&str> = s.split('(').collect();
        let role = parts[1].trim_end_matches(')');
        let role: Role = role.parse().map_err(|_| E::custom("invalid role"))?;
        match parts.as_slice() {
            ["read", _] => Ok(Permission::Read(role)),
            ["write", _] => Ok(Permission::Write(role)),
            ["create", _] => Ok(Permission::Create(role)),
            ["delete", _] => Ok(Permission::Delete(role)),
            ["update", _] => Ok(Permission::Update(role)),
            _ => Err(E::custom("invalid permission")),
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
fn test_serde() {
    let perm = Permission::Read(Role::Any);
    let serialized = serde_json::to_string(&perm).unwrap();
    assert_eq!(serialized, r#""read(\"any\")""#);

    let deserialized: Permission = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, perm);
}
