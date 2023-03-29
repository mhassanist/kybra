import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/ethereum_json_rpc/test/tests';
import { createActor } from './dfx_generated/ethereum_json_rpc';

if (process.env.ETHEREUM_URL === undefined) {
    throw new Error(
        `No Ethereum URL set, did you set the ETHEREUM_URL environment variable?\nExample: ETHEREUM_URL=https://some-ethereum-provider-url.org npm test`
    );
}

const ethereumJsonRpcCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(ethereumJsonRpcCanister)));
