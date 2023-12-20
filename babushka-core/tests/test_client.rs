mod utilities;

#[cfg(test)]
mod shared_client_tests {
    use super::*;
    use babushka::client::Client;
    use redis::RedisConnectionInfo;
    use redis::Value;
    use rstest::rstest;
    use utilities::cluster::*;
    use utilities::*;

    enum BackingServer {
        Cmd(Option<RedisServer>),
        Cluster(Option<RedisCluster>),
    }

    struct TestBasics {
        server: BackingServer,
        client: Client,
    }

    async fn setup_test_basics(use_cluster: bool, configuration: TestConfiguration) -> TestBasics {
        if use_cluster {
            let cluster_basics = cluster::setup_test_basics_internal(configuration).await;
            TestBasics {
                server: BackingServer::Cluster(cluster_basics.cluster),
                client: cluster_basics.client,
            }
        } else {
            let test_basics = utilities::setup_test_basics_internal(&configuration).await;

            let connection_addr = test_basics
                .server
                .as_ref()
                .map(|server| server.get_client_addr())
                .unwrap_or(get_shared_server_address(configuration.use_tls));

            // TODO - this is a patch, handling the situation where the new server
            // still isn't available to connection. This should be fixed in [RedisServer].
            let client = repeat_try_create(|| async {
                Client::new(create_connection_request(
                    &[connection_addr.clone()],
                    &configuration,
                ))
                .await
                .ok()
            })
            .await;

            TestBasics {
                server: BackingServer::Cmd(test_basics.server),
                client,
            }
        }
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_send_set_and_get(
        #[values(false, true)] use_tls: bool,
        #[values(false, true)] use_cluster: bool,
    ) {
        block_on_all(async {
            let test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    use_tls,
                    shared_server: true,
                    ..Default::default()
                },
            )
            .await;
            let key = generate_random_string(6);
            send_set_and_get(test_basics.client.clone(), key.to_string()).await;
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_resp3_support(#[values(false, true)] use_cluster: bool) {
        block_on_all(async {
            let mut test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    shared_server: true,
                    connection_info: Some(RedisConnectionInfo {
                        use_resp3: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await;
            let hello: std::collections::HashMap<String, Value> = redis::from_redis_value(
                &test_basics
                    .client
                    .req_packed_command(&redis::cmd("HELLO"), None)
                    .await
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(hello.get("proto").unwrap(), &Value::Int(3));

            let mut cmd = redis::cmd("HSET");
            cmd.arg("hash").arg("foo").arg("baz");
            test_basics
                .client
                .req_packed_command(&cmd, None)
                .await
                .unwrap();
            let mut cmd = redis::cmd("HSET");
            cmd.arg("hash").arg("bar").arg("foobar");
            test_basics
                .client
                .req_packed_command(&cmd, None)
                .await
                .unwrap();

            let mut cmd = redis::cmd("HGETALL");
            cmd.arg("hash");
            let result = test_basics
                .client
                .req_packed_command(&cmd, None)
                .await
                .unwrap();

            assert_eq!(
                result,
                Value::Map(vec![
                    (
                        Value::BulkString("foo".as_bytes().to_vec()),
                        Value::BulkString("baz".as_bytes().to_vec())
                    ),
                    (
                        Value::BulkString("bar".as_bytes().to_vec()),
                        Value::BulkString("foobar".as_bytes().to_vec())
                    )
                ])
            );
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_client_handle_concurrent_workload(
        #[values(false, true)] use_tls: bool,
        #[values(false, true)] use_cluster: bool,
    ) {
        block_on_all(async {
            let test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    use_tls,
                    shared_server: true,
                    ..Default::default()
                },
            )
            .await;
            const NUMBER_OF_CONCURRENT_OPERATIONS: usize = 1000;

            let mut actions = Vec::with_capacity(NUMBER_OF_CONCURRENT_OPERATIONS);
            for index in 0..NUMBER_OF_CONCURRENT_OPERATIONS {
                actions.push(send_set_and_get(
                    test_basics.client.clone(),
                    format!("key{index}"),
                ));
            }
            futures::future::join_all(actions).await;
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_report_closing_when_server_closes(#[values(false, true)] use_cluster: bool) {
        block_on_all(async {
            let mut test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    request_timeout: Some(10000000),
                    ..Default::default()
                },
            )
            .await;
            let server = test_basics.server;
            drop(server);

            let get_result = send_get(&mut test_basics.client, "foobar")
                .await
                .unwrap_err();
            assert!(get_result.is_connection_dropped());
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_authenticate_with_password(#[values(false, true)] use_cluster: bool) {
        block_on_all(async {
            let test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    use_tls: true,
                    connection_info: Some(redis::RedisConnectionInfo {
                        password: Some("ReallySecurePassword".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await;
            let key = generate_random_string(6);
            send_set_and_get(test_basics.client.clone(), key.to_string()).await;
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_authenticate_with_password_and_username(#[values(false, true)] use_cluster: bool) {
        block_on_all(async {
            let test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    use_tls: true,
                    connection_info: Some(redis::RedisConnectionInfo {
                        password: Some("ReallySecurePassword".to_string()),
                        username: Some("AuthorizedUsername".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await;
            let key = generate_random_string(6);
            send_set_and_get(test_basics.client.clone(), key.to_string()).await;
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_request_timeout(#[values(false, true)] use_cluster: bool) {
        block_on_all(async {
            let mut test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    request_timeout: Some(1),
                    shared_server: true,
                    ..Default::default()
                },
            )
            .await;

            let mut cmd = redis::Cmd::new();
            cmd.arg("BLPOP").arg("foo").arg(0); // 0 timeout blocks indefinitely
            let result = test_basics.client.req_packed_command(&cmd, None).await;
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.is_timeout(), "{err}");
        });
    }

    #[rstest]
    #[timeout(SHORT_CLUSTER_TEST_TIMEOUT)]
    fn test_request_pipeline_timeout(#[values(false, true)] use_cluster: bool) {
        block_on_all(async {
            let mut test_basics = setup_test_basics(
                use_cluster,
                TestConfiguration {
                    request_timeout: Some(1),
                    shared_server: true,
                    ..Default::default()
                },
            )
            .await;

            let mut pipeline = redis::pipe();
            pipeline.blpop("foo", 0.0); // 0 timeout blocks indefinitely
            let result = test_basics
                .client
                .req_packed_commands(&pipeline, 0, 1, None)
                .await;
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.is_timeout(), "{err}");
        });
    }
}
