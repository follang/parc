//! Contract tests proving the SourcePackage is consumable by downstream crates.
//!
//! These tests simulate what LINC/GERC would do with the package:
//! iterate items, switch on types, access provenance, serialize/deserialize,
//! filter, merge, and build packages programmatically.

use crate::extract;
use crate::ir::*;

/// A typical C API header used as the fixture for all tests.
const API_HEADER: &str = r#"
typedef unsigned long size_t;
typedef int int32_t;
typedef unsigned int uint32_t;

struct context;

struct config {
    const char *name;
    uint32_t flags;
    int32_t timeout_ms;
};

enum error {
    ERR_OK = 0,
    ERR_NOMEM = 1,
    ERR_IO = 2,
    ERR_INVAL = 3
};

struct context *ctx_create(const struct config *cfg);
void ctx_destroy(struct context *ctx);
enum error ctx_process(struct context *ctx, const void *data, size_t len);
const char *error_string(enum error err);

typedef void (*log_callback_t)(int level, const char *msg);
void set_logger(log_callback_t cb);

extern int verbose_level;
"#;

// --- Downstream iteration patterns ---

#[test]
fn consumable_iterate_all_items() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();

    let mut func_names = Vec::new();
    let mut record_names = Vec::new();
    let mut enum_names = Vec::new();
    let mut alias_names = Vec::new();
    let mut var_names = Vec::new();

    for item in &pkg.items {
        match item {
            SourceItem::Function(f) => func_names.push(f.name.clone()),
            SourceItem::Record(r) => {
                if let Some(name) = &r.name {
                    record_names.push(name.clone());
                }
            }
            SourceItem::Enum(e) => {
                if let Some(name) = &e.name {
                    enum_names.push(name.clone());
                }
            }
            SourceItem::TypeAlias(t) => alias_names.push(t.name.clone()),
            SourceItem::Variable(v) => var_names.push(v.name.clone()),
            SourceItem::Unsupported(_) => {}
        }
    }

    assert!(func_names.contains(&"ctx_create".to_string()));
    assert!(func_names.contains(&"ctx_destroy".to_string()));
    assert!(func_names.contains(&"ctx_process".to_string()));
    assert!(record_names.contains(&"context".to_string()));
    assert!(record_names.contains(&"config".to_string()));
    assert!(enum_names.contains(&"error".to_string()));
    assert!(alias_names.contains(&"size_t".to_string()));
    assert!(var_names.contains(&"verbose_level".to_string()));
}

// --- Type navigation ---

#[test]
fn consumable_navigate_function_types() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();
    let create = pkg.find_function("ctx_create").unwrap();

    // Return type: struct context *
    match &create.return_type {
        SourceType::Pointer { pointee, .. } => {
            assert_eq!(**pointee, SourceType::RecordRef("context".into()));
        }
        other => panic!("expected Pointer, got {:?}", other),
    }

    // First param: const struct config *
    assert_eq!(create.parameters.len(), 1);
    assert_eq!(
        create.parameters[0].ty,
        SourceType::const_ptr(SourceType::RecordRef("config".into()))
    );
}

#[test]
fn consumable_navigate_struct_fields() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();
    let config = pkg.find_record("config").unwrap();
    let fields = config.fields.as_ref().unwrap();

    // Downstream can iterate fields and switch on type
    for field in fields {
        let _name = field.name.as_deref().unwrap();
        match &field.ty {
            SourceType::Pointer { .. } => {} // string field
            SourceType::TypedefRef(name) => {
                assert!(name == "uint32_t" || name == "int32_t");
            }
            other => panic!("unexpected field type: {:?}", other),
        }
    }
}

#[test]
fn consumable_navigate_enum_variants() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();
    let err = pkg.find_enum("error").unwrap();

    // Downstream can build a variant -> value map
    let variant_map: std::collections::HashMap<&str, Option<i128>> = err
        .variants
        .iter()
        .map(|v| (v.name.as_str(), v.value))
        .collect();

    assert_eq!(variant_map["ERR_OK"], Some(0));
    assert_eq!(variant_map["ERR_INVAL"], Some(3));
}

// --- Serialization contract ---

#[test]
fn consumable_json_roundtrip() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();
    let json = serde_json::to_string_pretty(&pkg).unwrap();

    // Downstream deserializes from JSON
    let restored: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg, restored);

    // Items are preserved
    assert_eq!(restored.function_count(), pkg.function_count());
    assert_eq!(restored.record_count(), pkg.record_count());
    assert_eq!(restored.enum_count(), pkg.enum_count());
}

#[test]
fn consumable_json_is_readable() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();
    let json = serde_json::to_string_pretty(&pkg).unwrap();

    // JSON should contain recognizable field names
    assert!(json.contains("\"name\""));
    assert!(json.contains("\"return_type\""));
    assert!(json.contains("\"parameters\""));
    assert!(json.contains("\"fields\""));
    assert!(json.contains("\"variants\""));
}

// --- Filtering ---

#[test]
fn consumable_filter_functions_only() {
    let mut pkg = extract::extract_from_source(API_HEADER).unwrap();
    let original_count = pkg.item_count();

    pkg.retain_items(|item| item.is_function());
    assert!(pkg.item_count() < original_count);
    assert_eq!(pkg.item_count(), pkg.function_count());
    assert_eq!(pkg.record_count(), 0);
}

// --- Merge ---

#[test]
fn consumable_merge_two_headers() {
    let src1 = "int module_a_init(void);\nvoid module_a_cleanup(void);\n";
    let src2 = "int module_b_start(void);\nvoid module_b_stop(void);\n";

    let mut pkg1 = extract::extract_from_source(src1).unwrap();
    let pkg2 = extract::extract_from_source(src2).unwrap();

    pkg1.merge(pkg2);
    assert_eq!(pkg1.function_count(), 4);
    assert!(pkg1.find_function("module_a_init").is_some());
    assert!(pkg1.find_function("module_b_stop").is_some());
}

// --- Builder ---

#[test]
fn consumable_builder_pattern() {
    let pkg = SourcePackageBuilder::new()
        .source_path("generated.h")
        .item(SourceItem::Function(SourceFunction {
            name: "custom_fn".into(),
            calling_convention: CallingConvention::C,
            parameters: vec![SourceParameter {
                name: Some("x".into()),
                ty: SourceType::Int,
            }],
            return_type: SourceType::Void,
            variadic: false,
            source_offset: None,
        }))
        .item(SourceItem::TypeAlias(SourceTypeAlias {
            name: "my_int".into(),
            target: SourceType::Int,
            source_offset: None,
        }))
        .build();

    assert_eq!(pkg.source_path.as_deref(), Some("generated.h"));
    assert_eq!(pkg.function_count(), 1);
    assert_eq!(pkg.type_alias_count(), 1);

    // Should serialize cleanly
    let json = serde_json::to_string(&pkg).unwrap();
    let _: SourcePackage = serde_json::from_str(&json).unwrap();
}

// --- Item accessor patterns ---

#[test]
fn consumable_item_accessors() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();

    for item in &pkg.items {
        // Every item has a kind
        let _kind = item.kind();

        // Most items have names
        if let Some(name) = item.name() {
            assert!(!name.is_empty());
        }

        // as_* accessors work
        if item.is_function() {
            assert!(item.as_function().is_some());
            assert!(item.as_record().is_none());
        }
    }
}

// --- Diagnostics consumption ---

#[test]
fn consumable_diagnostics_usable() {
    let src = r#"
static int helper(void) { return 0; }
inline void fast(void) { }
int public_fn(void);
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    // Downstream can filter diagnostics by severity or kind
    let warnings: Vec<_> = pkg
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();
    assert!(!warnings.is_empty());

    // Diagnostic messages are human-readable
    for d in &pkg.diagnostics {
        assert!(!d.message.is_empty());
    }
}

// --- Count helpers ---

#[test]
fn consumable_count_helpers() {
    let pkg = extract::extract_from_source(API_HEADER).unwrap();

    let total = pkg.function_count()
        + pkg.record_count()
        + pkg.enum_count()
        + pkg.type_alias_count()
        + pkg.variable_count()
        + pkg.unsupported_count();

    assert_eq!(total, pkg.item_count());
}
