import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/heartbeat/test/tests';
import { createActor as createActorHeartbeatAsync } from './dfx_generated/heartbeat_async';
import { createActor as createActorHeartbeatSync } from './dfx_generated/heartbeat_sync';

const heartbeatAsyncCanister = createActorHeartbeatAsync(
    getCanisterId('heartbeat_async'),
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

const heartbeatSyncCanister = createActorHeartbeatSync(
    getCanisterId('heartbeat_sync'),
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

runTests(
    getTests(
        createSnakeCaseProxy(heartbeatAsyncCanister),
        createSnakeCaseProxy(heartbeatSyncCanister)
    )
);
