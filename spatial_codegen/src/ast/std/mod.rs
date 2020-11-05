use crate::ast::ASTNode;
use crate::ast::Component;
use crate::ast::DataType;
use crate::ast::Member;
use crate::ast::PackageNode;
use crate::ast::SchemaFile;
use crate::ast::Type;
use crate::ast::AST;

fn generate_position_component() -> Component {
    Component {
        name: "Position".to_string(),
        id: 54,
        members: vec![Member {
            name: "coords".to_string(),
            m_type: DataType::UserDefined("Coordinates".to_string()),
            id: 1,
        }],
        commands: Vec::new(),
        events: Vec::new(),
    }
}

fn generate_coordinates_type() -> Type {
    Type {
        name: "Coordinates".to_string(),
        members: vec![
            Member {
                name: "x".to_string(),
                m_type: DataType::Double,
                id: 1,
            },
            Member {
                name: "y".to_string(),
                m_type: DataType::Double,
                id: 2,
            },
            Member {
                name: "z".to_string(),
                m_type: DataType::Double,
                id: 3,
            },
        ],
    }
}

fn generate_worker_attribute_set_type() -> Type {
    Type {
        name: "WorkerAttributeSet".to_string(),
        members: vec![Member {
            name: "attribute".to_string(),
            m_type: DataType::List(Box::new(DataType::String)),
            id: 1,
        }],
    }
}

fn generate_worker_requirement_set_type() -> Type {
    Type {
        name: "WorkerRequirementSet".to_string(),
        members: vec![Member {
            name: "attribute".to_string(),
            m_type: DataType::List(Box::new(DataType::UserDefined(
                "WorkerAttributeSet".to_string(),
            ))),
            id: 1,
        }],
    }
}

fn generate_entity_acl_component() -> Component {
    Component {
        name: "EntityAcl".to_string(),
        id: 50,
        members: vec![
            Member {
                name: "read_acl".to_string(),
                m_type: DataType::UserDefined("WorkerRequirementSet".to_string()),
                id: 1,
            },
            Member {
                name: "component_write_acl".to_string(),
                m_type: DataType::Map(
                    Box::new(DataType::Uint32),
                    Box::new(DataType::UserDefined("WorkerRequirementSet".to_string())),
                ),
                id: 1,
            },
        ],
        events: Vec::new(),
        commands: Vec::new(),
    }
}

fn generate_persistence_component() -> Component {
    Component {
        name: "Persistence".to_string(),
        id: 55,
        members: Vec::new(),
        events: Vec::new(),
        commands: Vec::new(),
    }
}

fn generate_metadata_component() -> Component {
    Component {
        name: "Metadata".to_string(),
        id: 53,
        members: vec![Member {
            name: "entity_type".to_string(),
            m_type: DataType::String,
            id: 1,
        }],
        events: Vec::new(),
        commands: Vec::new(),
    }
}

//TODO: Create interest https://documentation.improbable.io/sdks-and-data/docs/the-standard-schema-library#section-interest

fn generate_system_component() -> Component {
    Component {
        name: "System".to_string(),
        id: 59,
        members: Vec::new(),
        events: Vec::new(),
        commands: Vec::new(),
    }
}

// TODO Create Worker https://documentation.improbable.io/sdks-and-data/docs/the-standard-schema-library#section-worker

fn generate_player_identity_type() -> Type {
    Type {
        name: "PlayerIdentity".to_string(),
        members: vec![
            Member {
                name: "player_identifier".to_string(),
                m_type: DataType::String,
                id: 1,
            },
            Member {
                name: "provider".to_string(),
                m_type: DataType::String,
                id: 2,
            },
            Member {
                name: "metadata".to_string(),
                m_type: DataType::Bytes,
                id: 3,
            },
        ],
    }
}

fn generate_player_client_component() -> Component {
    Component {
        name: "PlayerClient".to_string(),
        id: 61,
        members: vec![Member {
            name: "player_identity".to_string(),
            m_type: DataType::UserDefined("PlayerIdentity".to_string()),
            id: 1,
        }],
        events: Vec::new(),
        commands: Vec::new(),
    }
}

fn generate_improbable_restricted_schema_file() -> SchemaFile {
    SchemaFile {
        package_name: vec!["improbable".to_string(), "restricted".to_string()],
        name: "standard_library".to_string(),
        components: vec![
            generate_system_component(),
            // Add Worker Component
            generate_player_client_component(),
        ],
        types: vec![
            // Add Connection Type
            // Add DisconnectRequest Type
            // Add DisconnectResponse Type
            generate_player_identity_type(),
        ],
        enums: Vec::new(),
    }
}

fn generate_improbabled_schema_file() -> SchemaFile {
    SchemaFile {
        package_name: vec!["improbable".to_string()],
        name: "standard_library".to_string(),
        components: vec![
            generate_position_component(),
            generate_entity_acl_component(),
            generate_persistence_component(),
            generate_metadata_component(),
            // Add Interest Component
        ],
        types: vec![
            generate_coordinates_type(),
            generate_worker_attribute_set_type(),
            generate_worker_requirement_set_type(),
            // Add ComponentInterest Type
        ],
        enums: Vec::new(),
    }
}

fn generate_improbable_restricted_package() -> PackageNode {
    PackageNode {
        name: "restricted".to_string(),
        inner: vec![ASTNode::SchemaNode(
            generate_improbable_restricted_schema_file(),
        )],
    }
}

fn generate_improbable_package() -> PackageNode {
    PackageNode {
        name: "improbable".to_string(),
        inner: vec![
            ASTNode::PackageNode(generate_improbable_restricted_package()),
            ASTNode::SchemaNode(generate_improbabled_schema_file()),
        ],
    }
}

pub fn generate_standard_library() -> AST {
    AST {
        inner: vec![ASTNode::PackageNode(generate_improbable_package())],
    }
}
