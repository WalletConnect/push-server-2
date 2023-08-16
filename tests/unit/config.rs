use {crate::ENV_MUTEX, push_server::config::DEFAULT_PORT};

/// Function that removes all set env vars, executes a function and then sets
/// them all back.
pub fn env_bubble(variables: Vec<(&str, &str)>, bubble: fn() -> ()) {
    let vars_before = std::env::vars();
    for (k, _v) in std::env::vars() {
        std::env::remove_var(k);
    }
    for (k, v) in variables.clone() {
        std::env::set_var(k, v);
    }
    bubble();
    for (k, _v) in variables {
        std::env::remove_var(k);
    }
    for (k, v) in vars_before {
        std::env::set_var(k, v);
    }
}

#[test]
fn config_err_missing_env_var() {
    let _gaurd = ENV_MUTEX.lock().unwrap();

    let config_result = push_server::config::get_config();

    assert!(config_result.is_err())
}

#[test]
/// Test to check that EnvConfig loads values correctly both from defaults and
/// from ENV This function should be updated when new values are added.
fn config_uses_correct_values() {
    let _gaurd = ENV_MUTEX.lock().unwrap();

    env_bubble(
        vec![
            ("DATABASE_URL", "postgres://localhost:5432/echo"),
            #[cfg(feature = "multitenant")]
            (
                "TENANT_DATABASE_URL",
                "postgres://localhost:5432/echo-tenants",
            ),
            ("PUBLIC_URL", "https://test.walletconnect.com"),
        ],
        || {
            let config_result = push_server::config::get_config();

            assert!(config_result.is_ok());

            let config = config_result.unwrap();

            // Default Values
            assert_eq!(config.port, DEFAULT_PORT);

            // Configured using ENV
            assert_eq!(config.database_url, "postgres://localhost:5432/echo");
            #[cfg(feature = "multitenant")]
            assert_eq!(
                config.tenant_database_url,
                "postgres://localhost:5432/echo-tenants"
            );
            assert_eq!(config.public_url, "https://test.walletconnect.com");
        },
    )
}
