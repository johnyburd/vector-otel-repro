
### Expected outcome
 Can use the opentelemetry and tracing crates to publish logs to vector

### Actual outcome
 ~~Vector returns a 500 with no error message ~~

### Resolution
* make sure to include `/v1/logs` when posting logs
* vector does not currently support the httpjson protocol https://github.com/vectordotdev/vector/pull/22875

# Setup

### Run vector
```
vector -vvv --config ./vector.toml
```

### Run sample client
```
cargo run
```


# Output
### Vector
```
> vector -vvv --config .\vector.toml
2025-04-22T20:41:56.727811Z DEBUG vector::app: Internal log rate limit configured. internal_log_rate_secs=10
2025-04-22T20:41:56.728214Z  INFO vector::app: Log level is enabled. level="trace"
2025-04-22T20:41:56.728726Z DEBUG vector::app: messaged="Building runtime." worker_threads=20
2025-04-22T20:41:56.730587Z  INFO vector::app: Loading configs. paths=["vector.toml"]
2025-04-22T20:41:56.731598Z DEBUG vector::config::loading: No secret placeholder found, skipping secret resolution.
2025-04-22T20:41:56.732462Z DEBUG vector::topology::builder: Building new source. component=my_source_id
2025-04-22T20:41:56.733556Z DEBUG vector::topology::builder: Building new sink. component=out
2025-04-22T20:41:56.934146Z  INFO vector::topology::running: Running healthchecks.
2025-04-22T20:41:56.934608Z DEBUG vector::topology::running: Connecting changed/added component(s).
2025-04-22T20:41:56.934854Z  INFO vector::topology::builder: Healthcheck passed.
2025-04-22T20:41:56.934904Z DEBUG vector::topology::running: Configuring outputs for source. component=my_source_id
2025-04-22T20:41:56.935554Z DEBUG vector::topology::running: Configuring output for component. component=my_source_id output_id=Some("logs")
2025-04-22T20:41:56.935767Z DEBUG vector::topology::running: Configuring output for component. component=my_source_id output_id=Some("traces")
2025-04-22T20:41:56.935968Z DEBUG vector::topology::running: Connecting inputs for sink. component=out
2025-04-22T20:41:56.936213Z DEBUG vector::topology::running: Adding component input to fanout. component=out fanout_id=my_source_id.logs
2025-04-22T20:41:56.936256Z DEBUG vector::topology::running: Adding component input to fanout. component=out fanout_id=my_source_id.traces
2025-04-22T20:41:56.936293Z DEBUG vector::topology::running: Spawning new source. key=my_source_id
2025-04-22T20:41:56.936414Z TRACE vector::topology::running: Spawning new sink. key=out
2025-04-22T20:41:56.936430Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source pump supervisor starting.
2025-04-22T20:41:56.936730Z  INFO vector: Vector has started. debug="false" version="0.46.1" arch="x86_64" revision="9a19e8a 2025-04-14 18:36:30.707862743"
2025-04-22T20:41:56.936702Z DEBUG sink{component_kind="sink" component_id=out component_type=console}: vector::topology::builder: Sink starting.
2025-04-22T20:41:56.936736Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source pump starting.
2025-04-22T20:41:56.936439Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source starting.
2025-04-22T20:41:56.936752Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source pump starting.
2025-04-22T20:41:56.939697Z  INFO source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::sources::util::grpc: Building gRPC server. address=0.0.0.0:4317
2025-04-22T20:41:56.939738Z  INFO vector::internal_events::api: API server running. address=127.0.0.1:8686 playground=http://127.0.0.1:8686/playground graphql=http://127.0.0.1:8686/graphql
2025-04-22T20:41:56.940218Z  INFO source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::sources::opentelemetry::http: Building HTTP server. address=0.0.0.0:4318
2025-04-22T20:41:56.943077Z DEBUG sink{component_kind="sink" component_id=out component_type=console}: vector::utilization: utilization=0.007171167539970491
2025-04-22T20:41:56.943140Z TRACE vector: Beep.
2025-04-22T20:41:57.952016Z TRACE vector: Beep.
2025-04-22T20:41:58.216181Z TRACE hyper::proto::h1::conn: Conn::read_head
2025-04-22T20:41:58.216417Z TRACE hyper::proto::h1::conn: flushed({role=server}): State { reading: Init, writing: Init, keep_alive: Busy }
2025-04-22T20:41:58.216789Z TRACE hyper::proto::h1::conn: Conn::read_head
2025-04-22T20:41:58.217064Z TRACE hyper::proto::h1::io: received 160 bytes
2025-04-22T20:41:58.217223Z TRACE parse_headers: hyper::proto::h1::role: Request.parse bytes=160
2025-04-22T20:41:58.217324Z TRACE parse_headers: hyper::proto::h1::role: Request.parse Complete(160)
2025-04-22T20:41:58.217373Z DEBUG hyper::proto::h1::io: parsed 5 headers
2025-04-22T20:41:58.217533Z DEBUG hyper::proto::h1::conn: incoming body is empty
2025-04-22T20:41:58.217856Z TRACE hyper::proto::h1::conn: {role=server}: prepare possible HTTP upgrade
2025-04-22T20:41:58.217957Z DEBUG http-request{method=GET path=/graphql}: vector::internal_events::http: Received HTTP request. internal_log_rate_limit=true
2025-04-22T20:41:58.218281Z TRACE http-request{method=GET path=/graphql}: warp::filters::path: "health"?: "graphql"
2025-04-22T20:41:58.218430Z TRACE http-request{method=GET path=/graphql}: warp::filters::path: "graphql"?: "graphql"
2025-04-22T20:41:58.218636Z TRACE http-request{method=GET path=/graphql}: warp::filters::method: method::GET?: GET
2025-04-22T20:41:58.218892Z TRACE http-request{method=GET path=/graphql}: warp::filters::header: header2("connection")
2025-04-22T20:41:58.219378Z TRACE http-request{method=GET path=/graphql}: warp::filters::header: exact_ignore_case("upgrade", "websocket")
2025-04-22T20:41:58.219720Z TRACE http-request{method=GET path=/graphql}: warp::filters::header: exact?("sec-websocket-version", "13")
2025-04-22T20:41:58.220179Z TRACE http-request{method=GET path=/graphql}: warp::filters::header: header2("sec-websocket-key")
2025-04-22T20:41:58.220564Z TRACE http-request{method=GET path=/graphql}: warp::filters::header: optional("sec-websocket-protocol")
2025-04-22T20:41:58.222013Z TRACE encode_headers: hyper::proto::h1::role: Server::encode status=101, body=None, req_method=Some(GET)
2025-04-22T20:41:58.222590Z TRACE encode_headers: hyper::proto::h1::role: server body forced to 0; method=Some(GET), status=101
2025-04-22T20:41:58.223141Z DEBUG hyper::proto::h1::io: flushed 202 bytes
2025-04-22T20:41:58.223386Z TRACE hyper::proto::h1::conn: State::close()
2025-04-22T20:41:58.223561Z TRACE hyper::proto::h1::conn: flushed({role=server}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2025-04-22T20:41:58.223715Z TRACE hyper::upgrade: pending upgrade fulfill
2025-04-22T20:41:58.224014Z TRACE warp::filters::ws: websocket upgrade complete
2025-04-22T20:41:58.224136Z TRACE tokio_tungstenite::handshake: Setting context when skipping handshake
2025-04-22T20:41:58.224217Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.224329Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.224399Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.224460Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:149 Read.read
2025-04-22T20:41:58.224521Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.224582Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:152 Read.with_context read -> poll_read
2025-04-22T20:41:58.224811Z TRACE tokio_tungstenite::compat: WouldBlock
2025-04-22T20:41:58.224944Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.225021Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:178 Write.flush
2025-04-22T20:41:58.225088Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.225187Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:180 Write.with_context flush -> poll_flush
2025-04-22T20:41:58.225284Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.225348Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.225411Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.225473Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:149 Read.read
2025-04-22T20:41:58.225684Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.225755Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:152 Read.with_context read -> poll_read
2025-04-22T20:41:58.225823Z TRACE tungstenite::protocol::frame::frame: Parsed headers [129, 154]
2025-04-22T20:41:58.225922Z TRACE tungstenite::protocol::frame::frame: First: 10000001
2025-04-22T20:41:58.226013Z TRACE tungstenite::protocol::frame::frame: Second: 10011010
2025-04-22T20:41:58.226075Z TRACE tungstenite::protocol::frame::frame: Opcode: Data(Text)
2025-04-22T20:41:58.226142Z TRACE tungstenite::protocol::frame::frame: Masked: true
2025-04-22T20:41:58.226204Z TRACE tungstenite::protocol::frame: received frame
<FRAME>
final: true
reserved: false false false
opcode: TEXT
length: 32
payload length: 26
payload: 0x29b9d3a422fe85e770f8c8b33cfec4a93bf4c9823bf5cea970e6

2025-04-22T20:41:58.226280Z TRACE tungstenite::protocol: Received message {"type":"connection_init"}
2025-04-22T20:41:58.226356Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.226419Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.226481Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.226544Z TRACE tungstenite::protocol::frame::frame: Parsed headers [129, 254]
2025-04-22T20:41:58.226606Z TRACE tungstenite::protocol::frame::frame: First: 10000001
2025-04-22T20:41:58.226667Z TRACE tungstenite::protocol::frame::frame: Second: 11111110
2025-04-22T20:41:58.226729Z TRACE tungstenite::protocol::frame::frame: Opcode: Data(Text)
2025-04-22T20:41:58.226842Z TRACE tungstenite::protocol::frame::frame: Masked: true
2025-04-22T20:41:58.226996Z TRACE tungstenite::protocol::frame: received frame
<FRAME>
final: true
reserved: false false false
opcode: TEXT
length: 1281
payload length: 1273
payload: 0xc9cbb62990d3fd2bd6d9e87587d9ec60d38be9299fddea7a8bc4be7a87d9f229d08aeb75868cbe2e8adee66f9ecbab34c28cfd77909aab2cc09dfd619099be34de86be2990d3a46fc488ad24d38bb328c1cbe5369086aa39c29cab3ee288ab39d79bb13e90d384109ecbb623c29cab3ee288ab39d79bb13e90d3846fdd9cab6fefc5fd21db84b63990d3ee7d82c5fd24dc9dba3fc488b36f88dcef7d9ecbba23d186bb24dc8efd7790a38c02fccba2619098aa28c090fd77909aaa2fc18aad24c29db622dcc99038c699aa39f79fba23c69a9d34f186b23ddd87ba23c6a0bb1dd39dab28c087ac1ec78bac2ec080af39db86b165ee9b832392c9ff6d9686aa39c29cab3ee288ab39d79bb13e88c9841ec69bb623d5c8826c9ec9fb24dc99aa39c1b9be39c68cad23c1d3ff16e19dad24dc8efe109ec9fb21db84b63988c99623c6c8f36d9680b139d79ba92cded3ff04dc9dfe6192cdba23d186bb24dc8ee56df79fba23c6acb12edd8db623d5bda63dd7c8f636ee9b832392c9ff6ddd9cab3dc79d9a3bd787ab3ef0909c22df99b023d787ab04d6b9be39c68cad23c1c1b038c699aa39c1b9be39c68cad23c1d3ff69dd9cab3dc79dac1dd39dab28c087ac619280b13dc79dac1dd39dab28c087ac7792cdb623c29cab3ee288ab39d79bb13e9ec9b324df80ab7792cdb324df80ab619280b139d79ba92cded3ff69db87ab28c09fbe219bc9a411c0b5b16d92c9ff6d92c9ff12ed9da63dd787be20d7b5ad11dcc9ff6d92c9ff6d92c7f1639286b16dfe86b86dc9b5ad11dcc9ff6d92c9ff6d92c9ff6d928ab020c286b128dc9d9629ee9b832392c9ff6d92c9ff6d92c9ff6dd186b23ddd87ba23c6bda63dd7b5ad11dcc9ff6d92c9ff6d92c9ff6d928ab020c286b128dc9d9424dc8d833fee87ff6d92c9ff6d92c9ff6d92c9b228c19abe2ad7b5ad11dcc9ff6d92c9ff6d92c9ff6d929db620d79aab2cdf99833fee87ff6d92c9ff6d92c9ff6d92c9ac39c080b12a9a8cb12edd8db623d5d3ff69d787bc22d680b12a9bb5ad11dcc9ff6d92c9ff6d9294833fee87ff6d92c9ff6d92c9f1639cc9b02392a4ba39c080bc6dc9b5ad11dcc9ff6d92c9ff6d92c9ff6d928ab020c286b128dc9d9629ee9b832392c9ff6d92c9ff6d92c9ff6dd186b23ddd87ba23c6bda63dd7b5ad11dcc9ff6d92c9ff6d92c9ff6d928ab020c286b128dc9d9424dc8d833fee87ff6d92c9ff6d92c9ff6d92c9ab24df8cac39d384af11c0b5b16d92c9ff6d92c9ff6d92c9ff3ec69bb623d5c1ba23d186bb24dc8ee56d968cb12edd8db623d5c0833fee87ff6d92c9ff6d92c9a211c0b5b16d92c9ff6d92c9ff639cc7ff22dcc98b3fd38aba6dc9b5ad11dcc9ff6d92c9ff6d92c9ff6d928ab020c286b128dc9d9629ee9b832392c9ff6d92c9ff6d92c9ff6dd186b23ddd87ba23c6bda63dd7b5ad11dcc9ff6d92c9ff6d92c9ff6d928ab020c286b128dc9d9424dc8d833fee87ff6d92c9ff6d92c9ff6d92c9ac39c080b12a9a8cb12edd8db623d5d3ff69d787bc22d680b12a9bb5ad11dcc9ff6d92c9ff6d9294833fee87ff6d92c9ff6d92c9f1639cc9b02392aca928dc9d9122c680b924d188ab24dd87ff36ee9b832392c9ff6d92c9ff6d92c9ff6ddf8cac3ed38eba11c0b5b16d92c9ff6d92c9ff30ee9b832392c9ff6dcfb5ad11dc94833fee87fd619086af28c088ab24dd87912cdf8cfd7790a6aa39c29cab08c48cb139c1aba60edd84af22dc8cb139fb8d8f2cc69dba3fdc9a8c38d09abc3fdb99ab24dd87fd30cf

2025-04-22T20:41:58.227251Z TRACE tungstenite::protocol: Received message {"id":"fd078503-ab6d-4579-a750-dbc484eac879","type":"start","payload":{"variables":{"outputsPatterns":[],"inputsPatterns":["out"],"limit":100,"interval":500,"encoding":"JSON"},"query":"subscription OutputEventsByComponentIdPatternsSubscription(\r\n    $outputsPatterns: [String!]!, $inputsPatterns: [String!], $limit: Int!, $interval: Int!, $encoding: EventEncodingType!){\r\n    outputEventsByComponentIdPatterns(outputsPatterns: $outputsPatterns, inputsPatterns: $inputsPatterns, limit: $limit, interval: $interval) {\r\n        __typename\r\n        ... on Log {\r\n            componentId\r\n            componentType\r\n            componentKind\r\n            message\r\n            timestamp\r\n            string(encoding: $encoding)\r\n        }\r\n        ... on Metric {\r\n            componentId\r\n            componentType\r\n            componentKind\r\n            timestamp\r\n            string(encoding: $encoding)\r\n        }\r\n        ... on Trace {\r\n            componentId\r\n            componentType\r\n            componentKind\r\n            string(encoding: $encoding)\r\n        }\r\n        ... on EventNotification {\r\n            message\r\n        }\r\n    }\r\n}\r\n","operationName":"OutputEventsByComponentIdPatternsSubscription"}}
2025-04-22T20:41:58.227395Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.227527Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.227699Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.227851Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:149 Read.read
2025-04-22T20:41:58.227985Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.228114Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:152 Read.with_context read -> poll_read
2025-04-22T20:41:58.228220Z TRACE tokio_tungstenite::compat: WouldBlock
2025-04-22T20:41:58.228562Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.228678Z TRACE tungstenite::protocol: Sending frame: Frame { header: FrameHeader { is_final: true, rsv1: false, rsv2: false, rsv3: false, opcode: Data(Text), mask: None }, payload: [123, 34, 116, 121, 112, 101, 34, 58, 34, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 95, 97, 99, 107, 34, 125] }
2025-04-22T20:41:58.228755Z TRACE tungstenite::protocol::frame: writing frame
<FRAME>
final: true
reserved: false false false
opcode: TEXT
length: 27
payload length: 25
payload: 0x7b2274797065223a22636f6e6e656374696f6e5f61636b227d

2025-04-22T20:41:58.228831Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.228894Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:167 Write.write
2025-04-22T20:41:58.228958Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.229039Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:169 Write.with_context write -> poll_write
2025-04-22T20:41:58.229171Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:178 Write.flush
2025-04-22T20:41:58.229253Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.229317Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:180 Write.with_context flush -> poll_flush
2025-04-22T20:41:58.229412Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.229481Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.229618Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.229682Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:149 Read.read
2025-04-22T20:41:58.229749Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.229814Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:152 Read.with_context read -> poll_read
2025-04-22T20:41:58.229430Z DEBUG tap_handler{component_kind="sink" component_id="_tap" component_type="tap"}: vector_tap::controller: Started tap. outputs_patterns={} inputs_patterns={"out"}
2025-04-22T20:41:58.229881Z TRACE tokio_tungstenite::compat: WouldBlock
2025-04-22T20:41:58.230016Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.229966Z DEBUG tap_handler{component_kind="sink" component_id="_tap" component_type="tap"}: vector_tap::controller: Component matched. output.output_id=OutputId { component: ComponentKey { id: "my_source_id" }, port: Some("traces") } component_id_patterns={InputPattern("out", [Pattern { original: "my_source_id.logs", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('l'), Char('o'), Char('g'), Char('s')], is_recursive: false }, Pattern { original: "my_source_id.traces", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('t'), Char('r'), Char('a'), Char('c'), Char('e'), Char('s')], is_recursive: false }])} matched=[InputPattern("out", [Pattern { original: "my_source_id.logs", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('l'), Char('o'), Char('g'), Char('s')], is_recursive: false }, Pattern { original: "my_source_id.traces", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('t'), Char('r'), Char('a'), Char('c'), Char('e'), Char('s')], is_recursive: false }])]
2025-04-22T20:41:58.230124Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:178 Write.flush
2025-04-22T20:41:58.230283Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.230359Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:180 Write.with_context flush -> poll_flush
2025-04-22T20:41:58.230234Z DEBUG tap_handler{component_kind="sink" component_id="_tap" component_type="tap"}: vector_tap::controller: Sink connected. sink_id="cb0edb6d-2c4b-4fdf-afb2-53fba08698eb" output.output_id=OutputId { component: ComponentKey { id: "my_source_id" }, port: Some("traces") }
2025-04-22T20:41:58.230513Z DEBUG tap_handler{component_kind="sink" component_id="_tap" component_type="tap"}: vector_tap::controller: Component matched. output.output_id=OutputId { component: ComponentKey { id: "my_source_id" }, port: Some("logs") } component_id_patterns={InputPattern("out", [Pattern { original: "my_source_id.logs", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('l'), Char('o'), Char('g'), Char('s')], is_recursive: false }, Pattern { original: "my_source_id.traces", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('t'), Char('r'), Char('a'), Char('c'), Char('e'), Char('s')], is_recursive: false }])} matched=[InputPattern("out", [Pattern { original: "my_source_id.logs", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('l'), Char('o'), Char('g'), Char('s')], is_recursive: false }, Pattern { original: "my_source_id.traces", tokens: [Char('m'), Char('y'), Char('_'), Char('s'), Char('o'), Char('u'), Char('r'), Char('c'), Char('e'), Char('_'), Char('i'), Char('d'), Char('.'), Char('t'), Char('r'), Char('a'), Char('c'), Char('e'), Char('s')], is_recursive: false }])]
2025-04-22T20:41:58.230605Z DEBUG tap_handler{component_kind="sink" component_id="_tap" component_type="tap"}: vector_tap::controller: Sink connected. sink_id="e282909d-b019-4901-bc3c-83c0a09eace9" output.output_id=OutputId { component: ComponentKey { id: "my_source_id" }, port: Some("logs") }
2025-04-22T20:41:58.230677Z DEBUG tap_handler{component_kind="sink" component_id="_tap" component_type="tap"}: vector_tap::controller: Sending matched notification. pattern="out"
2025-04-22T20:41:58.230764Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.230916Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.230978Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.231040Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:149 Read.read
2025-04-22T20:41:58.231103Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.231167Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:152 Read.with_context read -> poll_read
2025-04-22T20:41:58.231425Z TRACE tokio_tungstenite::compat: WouldBlock
2025-04-22T20:41:58.231717Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.231958Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.232123Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.232215Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:149 Read.read
2025-04-22T20:41:58.232302Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.232375Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:152 Read.with_context read -> poll_read
2025-04-22T20:41:58.232455Z TRACE tokio_tungstenite::compat: WouldBlock
2025-04-22T20:41:58.232532Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.232596Z TRACE tungstenite::protocol: Sending frame: Frame { header: FrameHeader { is_final: true, rsv1: false, rsv2: false, rsv3: false, opcode: Data(Text), mask: None }, payload: [123, 34, 116, 121, 112, 101, 34, 58, 34, 100, 97, 116, 97, 34, 44, 34, 105, 100, 34, 58, 34, 102, 100, 48, 55, 56, 53, 48, 51, 45, 97, 98, 54, 100, 45, 52, 53, 55, 57, 45, 97, 55, 53, 48, 45, 100, 98, 99, 52, 56, 52, 101, 97, 99, 56, 55, 57, 34, 44, 34, 112, 97, 121, 108, 111, 97, 100, 34, 58, 123, 34, 100, 97, 116, 97, 34, 58, 123, 34, 111, 117, 116, 112, 117, 116, 69, 118, 101, 110, 116, 115, 66, 121, 67, 111, 109, 112, 111, 110, 101, 110, 116, 73, 100, 80, 97, 116, 116, 101, 114, 110, 115, 34, 58, 91, 123, 34, 95, 95, 116, 121, 112, 101, 110, 97, 109, 101, 34, 58, 34, 69, 118, 101, 110, 116, 78, 111, 116, 105, 102, 105, 99, 97, 116, 105, 111, 110, 34, 44, 34, 109, 101, 115, 115, 97, 103, 101, 34, 58, 34, 91, 116, 97, 112, 93, 32, 80, 97, 116, 116, 101, 114, 110, 32, 39, 111, 117, 116, 39, 32, 115, 117, 99, 99, 101, 115, 115, 102, 117, 108, 108, 121, 32, 109, 97, 116, 99, 104, 101, 100, 46, 34, 125, 93, 125, 125, 125] }
2025-04-22T20:41:58.232672Z TRACE tungstenite::protocol::frame: writing frame
<FRAME>
final: true
reserved: false false false
opcode: TEXT
length: 211
payload length: 207
payload: 0x7b2274797065223a2264617461222c226964223a2266643037383530332d616236642d343537392d613735302d646263343834656163383739222c227061796c6f6164223a7b2264617461223a7b226f75747075744576656e74734279436f6d706f6e656e7449645061747465726e73223a5b7b225f5f747970656e616d65223a224576656e744e6f74696669636174696f6e222c226d657373616765223a225b7461705d205061747465726e20276f757427207375636365737366756c6c79206d6174636865642e227d5d7d7d7d

2025-04-22T20:41:58.232744Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.232805Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:167 Write.write
2025-04-22T20:41:58.232870Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.232941Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:169 Write.with_context write -> poll_write
2025-04-22T20:41:58.233073Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:178 Write.flush
2025-04-22T20:41:58.233146Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.233222Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:180 Write.with_context flush -> poll_flush
2025-04-22T20:41:58.233293Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:288 Stream.poll_next
2025-04-22T20:41:58.233367Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.233431Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:298 Stream.with_context poll_next -> read()
2025-04-22T20:41:58.233493Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:149 Read.read
2025-04-22T20:41:58.233560Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.233633Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:152 Read.with_context read -> poll_read
2025-04-22T20:41:58.233703Z TRACE tokio_tungstenite::compat: WouldBlock
2025-04-22T20:41:58.233766Z TRACE tokio_tungstenite: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\lib.rs:243 WebSocketStream.with_context
2025-04-22T20:41:58.233828Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:178 Write.flush
2025-04-22T20:41:58.233974Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:126 AllowStd.with_context
2025-04-22T20:41:58.234036Z TRACE tokio_tungstenite::compat: C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-tungstenite-0.21.0\src\compat.rs:180 Write.with_context flush -> poll_flush
2025-04-22T20:41:58.854360Z TRACE hyper::proto::h1::conn: Conn::read_head
2025-04-22T20:41:58.854672Z TRACE hyper::proto::h1::conn: flushed({role=server}): State { reading: Init, writing: Init, keep_alive: Busy }
2025-04-22T20:41:58.855563Z TRACE hyper::proto::h1::conn: Conn::read_head
2025-04-22T20:41:58.855737Z TRACE hyper::proto::h1::io: received 639 bytes
2025-04-22T20:41:58.855805Z TRACE parse_headers: hyper::proto::h1::role: Request.parse bytes=639
2025-04-22T20:41:58.855835Z TRACE parse_headers: hyper::proto::h1::role: Request.parse Complete(157)
2025-04-22T20:41:58.855869Z DEBUG hyper::proto::h1::io: parsed 5 headers
2025-04-22T20:41:58.855893Z DEBUG hyper::proto::h1::conn: incoming body is content-length (482 bytes)
2025-04-22T20:41:58.855942Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}:http-request{method=POST path=/}: vector::internal_events::http: Received HTTP request. internal_log_rate_limit=true
2025-04-22T20:41:58.856057Z TRACE source{component_kind="source" component_id=my_source_id component_type=opentelemetry}:http-request{method=POST path=/}: warp::filters::method: method::POST?: POST
2025-04-22T20:41:58.856103Z TRACE hyper::proto::h1::decode: decode; state=Length(482)
2025-04-22T20:41:58.856178Z DEBUG hyper::proto::h1::conn: incoming body completed
2025-04-22T20:41:58.856331Z TRACE source{component_kind="source" component_id=my_source_id component_type=opentelemetry}:http-request{method=POST path=/}: warp::filters::path: "v1"?: ""
2025-04-22T20:41:58.856411Z TRACE source{component_kind="source" component_id=my_source_id component_type=opentelemetry}:http-request{method=POST path=/}: warp::filters::method: method::POST?: POST
2025-04-22T20:41:58.856585Z TRACE source{component_kind="source" component_id=my_source_id component_type=opentelemetry}:http-request{method=POST path=/}: warp::filters::path: "v1"?: ""
2025-04-22T20:41:58.856825Z TRACE encode_headers: hyper::proto::h1::role: Server::encode status=500, body=Some(Known(23)), req_method=Some(POST)
2025-04-22T20:41:58.856967Z TRACE hyper::proto::h1::encode: sized write, len = 23
2025-04-22T20:41:58.857088Z TRACE hyper::proto::h1::io: buffer.flatten self.len=133 buf.len=23
2025-04-22T20:41:58.857308Z DEBUG hyper::proto::h1::io: flushed 156 bytes
2025-04-22T20:41:58.857376Z TRACE hyper::proto::h1::conn: flushed({role=server}): State { reading: Init, writing: Init, keep_alive: Idle }
2025-04-22T20:41:58.857474Z TRACE hyper::proto::h1::conn: Conn::read_head
2025-04-22T20:41:58.857622Z TRACE hyper::proto::h1::conn: flushed({role=server}): State { reading: Init, writing: Init, keep_alive: Idle }
2025-04-22T20:41:58.858760Z TRACE hyper::proto::h1::conn: Conn::read_head
2025-04-22T20:41:58.858845Z TRACE hyper::proto::h1::io: received 0 bytes
2025-04-22T20:41:58.859063Z TRACE hyper::proto::h1::io: parse eof
2025-04-22T20:41:58.859225Z TRACE hyper::proto::h1::conn: State::close_read()
2025-04-22T20:41:58.859302Z DEBUG hyper::proto::h1::conn: read eof
2025-04-22T20:41:58.859421Z TRACE hyper::proto::h1::conn: State::close_write()
2025-04-22T20:41:58.859452Z TRACE hyper::proto::h1::conn: State::close_read()
2025-04-22T20:41:58.859479Z TRACE hyper::proto::h1::conn: State::close_write()
2025-04-22T20:41:58.859574Z TRACE hyper::proto::h1::conn: flushed({role=server}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2025-04-22T20:41:58.859748Z TRACE hyper::proto::h1::conn: shut down IO complete
2025-04-22T20:41:58.946252Z TRACE vector: Beep.
2025-04-22T20:41:59.785947Z  INFO vector: Vector has stopped.
2025-04-22T20:41:59.786418Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: hyper::server::shutdown: signal received, starting graceful shutdown
2025-04-22T20:41:59.786722Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: hyper::server::shutdown: signal received, starting graceful shutdown
2025-04-22T20:41:59.786853Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source finished normally.
2025-04-22T20:41:59.787111Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source pump finished normally.
2025-04-22T20:41:59.787033Z  INFO vector::topology::running: Shutting down... Waiting on running components. remaining_components="out, my_source_id" time_remaining="59 seconds left"
2025-04-22T20:41:59.786855Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source pump finished normally.
2025-04-22T20:41:59.787678Z DEBUG source{component_kind="source" component_id=my_source_id component_type=opentelemetry}: vector::topology::builder: Source pump supervisor task finished normally.
2025-04-22T20:41:59.787688Z DEBUG sink{component_kind="sink" component_id=out component_type=console}: vector::topology::builder: Sink finished normally.
2025-04-22T20:41:59.787961Z DEBUG hyper::server::shutdown: signal received, starting graceful shutdown
```

### vector-otel-repro
```
> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target\debug\vector-otel-repro.exe`
2025-04-22T20:41:58.527753Z  INFO vector_otel_repro: Please log me
2025-04-22T20:41:58.528054Z DEBUG opentelemetry_sdk:  name="LoggerProvider.ShutdownInvokedByUser"
2025-04-22T20:41:58.528316Z DEBUG opentelemetry_sdk:  name="BatchLogProcessor.ExportingDueToShutdown"
2025-04-22T20:41:58.528631Z DEBUG opentelemetry-otlp:  name="HttpLogsClient.CallingExport"
2025-04-22T20:41:58.528860Z DEBUG opentelemetry-http:  name="ReqwestBlockingClient.Send"
2025-04-22T20:41:58.529199Z TRACE reqwest::blocking::wait: wait at most 10s
2025-04-22T20:41:58.529311Z TRACE hyper_util::client::legacy::pool: checkout waiting for idle connection: ("http", localhost:4318)
2025-04-22T20:41:58.529411Z TRACE reqwest::blocking::wait: (ThreadId(24)) park timeout 9.9999971s
2025-04-22T20:41:58.529654Z DEBUG reqwest::connect: starting new connection: http://localhost:4318/
2025-04-22T20:41:58.529898Z TRACE hyper_util::client::legacy::connect::http: Http::connect; scheme=Some("http"), host=Some("localhost"), port=Some(Port(4318))
2025-04-22T20:41:58.541265Z DEBUG hyper_util::client::legacy::connect::http: connecting to [::1]:4318
2025-04-22T20:41:58.853501Z DEBUG hyper_util::client::legacy::connect::http: connecting to 127.0.0.1:4318
2025-04-22T20:41:58.854237Z DEBUG hyper_util::client::legacy::connect::http: connected to 127.0.0.1:4318
2025-04-22T20:41:58.854661Z TRACE hyper_util::client::legacy::client: http1 handshake complete, spawning background dispatcher task
2025-04-22T20:41:58.854945Z TRACE hyper_util::client::legacy::pool: checkout dropped for ("http", localhost:4318)
2025-04-22T20:41:58.857467Z TRACE hyper_util::client::legacy::pool: put; add idle connection for ("http", localhost:4318)
2025-04-22T20:41:58.857670Z DEBUG hyper_util::client::legacy::pool: pooling idle connection for ("http", localhost:4318)
2025-04-22T20:41:58.857914Z ERROR opentelemetry_sdk:  name="BatchLogProcessor.ExportError" error="Operation failed: reqwest::Error { kind: Status(500), url: \"http://localhost:4318/\" }"
2025-04-22T20:41:58.858085Z TRACE reqwest::blocking::client: closing runtime thread (ThreadId(23))
2025-04-22T20:41:58.858276Z TRACE reqwest::blocking::client: signaled close for runtime thread (ThreadId(23))
2025-04-22T20:41:58.858304Z TRACE reqwest::blocking::client: (ThreadId(23)) Receiver is shutdown
2025-04-22T20:41:58.858574Z TRACE reqwest::blocking::client: (ThreadId(23)) end runtime::block_on
2025-04-22T20:41:58.859132Z TRACE reqwest::blocking::client: (ThreadId(23)) finished
2025-04-22T20:41:58.859551Z TRACE reqwest::blocking::client: closed runtime thread (ThreadId(23))
2025-04-22T20:41:58.859937Z DEBUG opentelemetry_sdk:  name="BatchLogProcessor.ThreadExiting" reason="ShutdownRequested"
2025-04-22T20:41:58.860151Z DEBUG opentelemetry_sdk:  name="BatchLogProcessor.ThreadStopped"
```
